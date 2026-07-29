import { create } from "zustand";
import {
  commands,
  type GeneratedPrompt,
  type PromptMode,
  type RefineAction,
} from "../ipc/bindings";

/**
 * Janela de coalescência do desfazer. Sem ela, cada tecla viraria um nível e desfazer ficaria
 * inútil; com ela, um trecho digitado de uma vez é um nível só.
 */
export const UNDO_COALESCE_MS = 600;

/** PRODUCT-SPEC §5.5 exige no mínimo 10 níveis; o teto evita segurar texto antigo sem limite. */
const UNDO_LIMIT = 50;

interface PromptState {
  generating: boolean;
  saving: boolean;
  /** Ação de refino em andamento (a UI desabilita os botões e mostra qual está rodando). */
  refining: RefineAction | null;
  prompt: GeneratedPrompt | null;
  /** Texto do editor — pode divergir de `prompt.content` até salvar. */
  content: string;
  undoStack: string[];
  lastSnapshotAt: number;
  /** Há edição ainda não persistida em `generated_prompts.content`. */
  dirty: boolean;
  fallbackReason: string | null;
  error: string | null;
  mode: PromptMode;

  setMode: (mode: PromptMode) => void;
  generate: (transcriptionId: number) => Promise<void>;
  generateFromTemplate: (transcriptionId: number, templateId: number) => Promise<void>;
  setContent: (content: string) => void;
  undo: () => void;
  revertToOriginal: () => void;
  refine: (action: RefineAction) => Promise<void>;
  /** Persiste a edição. Devolve o texto salvo — é o que vai para o clipboard. */
  save: () => Promise<string>;
  reset: () => void;
}

function adopt(prompt: GeneratedPrompt, fallbackReason: string | null) {
  return {
    prompt,
    content: prompt.content,
    fallbackReason,
    dirty: false,
    generating: false,
    refining: null,
    error: null,
  };
}

export const usePromptStore = create<PromptState>((set, get) => ({
  generating: false,
  saving: false,
  refining: null,
  prompt: null,
  content: "",
  undoStack: [],
  lastSnapshotAt: 0,
  dirty: false,
  fallbackReason: null,
  error: null,
  mode: "technical",

  setMode(mode) {
    set({ mode });
  },

  async generate(transcriptionId: number) {
    set({ generating: true, error: null, fallbackReason: null });
    try {
      const result = await commands.generatePrompt(transcriptionId, get().mode);
      if (result.status === "ok") {
        // Regenerar é uma ação desfazível: o texto anterior entra na pilha.
        set((s) => ({
          ...pushSnapshot(s),
          ...adopt(result.data.prompt, result.data.fallback_reason),
        }));
      } else {
        set({ error: result.error, generating: false });
      }
    } catch (e) {
      set({ error: e instanceof Error ? e.message : String(e), generating: false });
    }
  },

  async generateFromTemplate(transcriptionId: number, templateId: number) {
    set({ generating: true, error: null, fallbackReason: null });
    try {
      const result = await commands.generatePromptFromTemplate(transcriptionId, templateId);
      if (result.status === "ok") {
        set((s) => ({
          ...pushSnapshot(s),
          ...adopt(result.data.prompt, result.data.fallback_reason),
        }));
      } else {
        set({ error: result.error, generating: false });
      }
    } catch (e) {
      set({ error: e instanceof Error ? e.message : String(e), generating: false });
    }
  },

  setContent(content: string) {
    const state = get();
    if (content === state.content) return;

    const now = Date.now();
    const coalesce = now - state.lastSnapshotAt < UNDO_COALESCE_MS && state.undoStack.length > 0;

    set({
      content,
      dirty: true,
      ...(coalesce ? {} : pushSnapshot(state)),
      lastSnapshotAt: now,
    });
  },

  undo() {
    const { undoStack } = get();
    const previous = undoStack[undoStack.length - 1];
    if (previous === undefined) return;

    set({
      content: previous,
      undoStack: undoStack.slice(0, -1),
      dirty: true,
      // Zera a coalescência: a próxima digitação abre um nível novo em vez de sobrescrever.
      lastSnapshotAt: 0,
    });
  },

  revertToOriginal() {
    const state = get();
    if (!state.prompt || state.content === state.prompt.original_content) return;
    set({
      ...pushSnapshot(state),
      content: state.prompt.original_content,
      dirty: true,
      lastSnapshotAt: 0,
    });
  },

  async refine(action: RefineAction) {
    const state = get();
    if (!state.prompt) return;
    const promptId = state.prompt.id;

    // O backend refina o que está no banco — salvar antes evita refinar uma versão velha.
    await state.save();

    set({ refining: action, error: null, fallbackReason: null });
    try {
      const result = await commands.refinePrompt(promptId, action);
      if (result.status === "ok") {
        set((s) => ({
          ...pushSnapshot(s),
          ...adopt(result.data.prompt, result.data.fallback_reason),
        }));
      } else {
        set({ error: result.error, refining: null });
      }
    } catch (e) {
      set({ error: e instanceof Error ? e.message : String(e), refining: null });
    }
  },

  async save() {
    const { prompt, content, dirty } = get();
    if (!prompt) return content;
    if (!dirty) return content;

    set({ saving: true });
    try {
      const result = await commands.updatePromptContent(prompt.id, content);
      if (result.status === "ok") {
        set({ prompt: result.data, dirty: false, saving: false });
      } else {
        set({ error: result.error, saving: false });
      }
    } catch (e) {
      set({ error: e instanceof Error ? e.message : String(e), saving: false });
    }
    return content;
  },

  reset() {
    set({
      generating: false,
      saving: false,
      refining: null,
      prompt: null,
      content: "",
      undoStack: [],
      lastSnapshotAt: 0,
      dirty: false,
      fallbackReason: null,
      error: null,
    });
  },
}));

/** Empilha o conteúdo atual, respeitando o teto de níveis. */
function pushSnapshot(state: Pick<PromptState, "content" | "undoStack">) {
  if (!state.content) return { undoStack: state.undoStack };
  const stack = [...state.undoStack, state.content];
  return { undoStack: stack.length > UNDO_LIMIT ? stack.slice(-UNDO_LIMIT) : stack };
}
