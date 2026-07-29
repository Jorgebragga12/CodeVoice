import { create } from "zustand";
import { commands, type GeneratedPrompt, type PromptMode } from "../ipc/bindings";

interface PromptState {
  generating: boolean;
  prompt: GeneratedPrompt | null;
  /** Aviso quando o Claude CLI falhou e o template foi usado. */
  fallbackReason: string | null;
  error: string | null;
  mode: PromptMode;

  setMode: (mode: PromptMode) => void;
  generate: (transcriptionId: number) => Promise<void>;
  reset: () => void;
}

export const usePromptStore = create<PromptState>((set, get) => ({
  generating: false,
  prompt: null,
  fallbackReason: null,
  error: null,
  mode: "technical",

  setMode(mode) {
    set({ mode });
  },

  async generate(transcriptionId: number) {
    set({ generating: true, error: null, fallbackReason: null, prompt: null });
    try {
      const result = await commands.generatePrompt(transcriptionId, get().mode);
      if (result.status === "ok") {
        set({
          prompt: result.data.prompt,
          fallbackReason: result.data.fallback_reason,
          generating: false,
        });
      } else {
        set({ error: result.error, generating: false });
      }
    } catch (e) {
      set({ error: e instanceof Error ? e.message : String(e), generating: false });
    }
  },

  reset() {
    set({ generating: false, prompt: null, fallbackReason: null, error: null });
  },
}));
