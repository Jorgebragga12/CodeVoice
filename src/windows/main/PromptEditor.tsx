import { useEffect, useState } from "react";
import type { RefineAction } from "../../ipc/bindings";
import { usePromptStore } from "../../stores/promptStore";
import { useTranscriptionStore } from "../../stores/transcriptionStore";
import { RefineToolbar } from "./RefineToolbar";
import { SaveAsTemplateDialog } from "./SaveAsTemplateDialog";

/** Persistência automática da edição livre, para não depender de o usuário clicar em copiar. */
const AUTOSAVE_MS = 1200;

/**
 * Editor da Fase 7 (PRODUCT-SPEC §5.5): transcrição original somente-leitura ao lado do prompt
 * editável, com refino, desfazer e "salvar como modelo".
 *
 * Copiar sempre persiste antes: o clipboard e `generated_prompts.content` não podem divergir.
 */
export function PromptEditor() {
  const transcript = useTranscriptionStore((s) => s.text);

  const prompt = usePromptStore((s) => s.prompt);
  const content = usePromptStore((s) => s.content);
  const setContent = usePromptStore((s) => s.setContent);
  const undo = usePromptStore((s) => s.undo);
  const revertToOriginal = usePromptStore((s) => s.revertToOriginal);
  const refine = usePromptStore((s) => s.refine);
  const save = usePromptStore((s) => s.save);
  const undoStack = usePromptStore((s) => s.undoStack);
  const refining = usePromptStore((s) => s.refining);
  const generating = usePromptStore((s) => s.generating);
  const saving = usePromptStore((s) => s.saving);
  const dirty = usePromptStore((s) => s.dirty);
  const fallbackReason = usePromptStore((s) => s.fallbackReason);

  const [copied, setCopied] = useState(false);
  const [savingTemplate, setSavingTemplate] = useState(false);
  const [templateSaved, setTemplateSaved] = useState<string | null>(null);

  // Autosave: o texto editado precisa sobreviver a fechar o app sem clicar em nada.
  useEffect(() => {
    if (!dirty) return;
    const timer = setTimeout(() => void save(), AUTOSAVE_MS);
    return () => clearTimeout(timer);
  }, [content, dirty, save]);

  if (!prompt) return null;

  const busy = generating || refining !== null;

  async function handleCopy() {
    const saved = await save();
    await navigator.clipboard.writeText(saved);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }

  async function handleOpenTemplateDialog() {
    // O diálogo salva o que está no banco — a edição pendente tem que ir junto.
    await save();
    setSavingTemplate(true);
  }

  return (
    <div className="flex flex-col gap-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <h3 className="text-sm font-medium text-zinc-300">
          Prompt
          <span className="ml-2 text-xs font-normal text-zinc-500">
            {prompt.generator === "claude_cli" ? "via Claude" : "via template"}
            {dirty ? " · editado" : ""}
            {saving ? " · salvando…" : ""}
          </span>
        </h3>
        <div className="flex items-center gap-1.5">
          <button
            type="button"
            onClick={() => void handleOpenTemplateDialog()}
            disabled={busy}
            className="rounded px-2.5 py-1 text-xs text-zinc-400 hover:text-zinc-200 disabled:opacity-50"
          >
            Salvar como modelo
          </button>
          <button
            type="button"
            onClick={() => void handleCopy()}
            disabled={busy}
            className="rounded bg-zinc-100 px-3 py-1 text-xs font-medium text-zinc-900 hover:bg-white disabled:opacity-50"
          >
            {copied ? "Copiado!" : "Copiar"}
          </button>
        </div>
      </div>

      {fallbackReason && <p className="text-xs text-amber-400">{fallbackReason}</p>}
      {templateSaved && (
        <p className="text-xs text-emerald-400">
          Modelo &ldquo;{templateSaved}&rdquo; salvo — está na aba Modelos.
        </p>
      )}

      <RefineToolbar
        refining={refining}
        busy={busy}
        canUndo={undoStack.length > 0}
        canRevert={content !== prompt.original_content}
        onRefine={(action: RefineAction) => void refine(action)}
        onUndo={undo}
        onRevert={revertToOriginal}
      />

      <div className="grid gap-3 lg:grid-cols-[1fr_1.6fr]">
        <div className="flex flex-col gap-1">
          <label htmlFor="editor-transcript" className="text-xs text-zinc-500">
            Transcrição original
          </label>
          <textarea
            id="editor-transcript"
            readOnly
            value={transcript ?? ""}
            rows={18}
            className="h-full w-full resize-y rounded border border-zinc-800 bg-zinc-950 p-2 text-xs text-zinc-400"
          />
        </div>

        <div className="flex flex-col gap-1">
          <label htmlFor="editor-prompt" className="text-xs text-zinc-500">
            Prompt (editável)
          </label>
          <textarea
            id="editor-prompt"
            value={content}
            onChange={(e) => setContent(e.target.value)}
            rows={18}
            spellCheck={false}
            className="h-full w-full resize-y rounded border border-zinc-700 bg-zinc-950 p-2 font-mono text-xs text-zinc-100"
          />
        </div>
      </div>

      <SaveAsTemplateDialog
        open={savingTemplate}
        promptId={prompt.id}
        onSaved={(name) => {
          setSavingTemplate(false);
          setTemplateSaved(name);
        }}
        onCancel={() => setSavingTemplate(false)}
      />
    </div>
  );
}
