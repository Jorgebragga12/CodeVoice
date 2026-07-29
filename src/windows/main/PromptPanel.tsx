import { useEffect, useState } from "react";
import { commands, type PromptMode, type PromptModeOption } from "../../ipc/bindings";
import { usePromptStore } from "../../stores/promptStore";
import { useTranscriptionStore } from "../../stores/transcriptionStore";

/**
 * Geração do prompt técnico a partir da transcrição (Fase 6): escolher o modo, gerar, e ver o
 * resultado. A edição e as ações de refino chegam na Fase 7.
 */
export function PromptPanel() {
  const transcriptionId = useTranscriptionStore((s) => s.transcriptionId);
  const transcriptionPhase = useTranscriptionStore((s) => s.phase);

  const mode = usePromptStore((s) => s.mode);
  const setMode = usePromptStore((s) => s.setMode);
  const generate = usePromptStore((s) => s.generate);
  const generating = usePromptStore((s) => s.generating);
  const prompt = usePromptStore((s) => s.prompt);
  const fallbackReason = usePromptStore((s) => s.fallbackReason);
  const error = usePromptStore((s) => s.error);

  const [modes, setModes] = useState<PromptModeOption[]>([]);
  const [cliAvailable, setCliAvailable] = useState<boolean | null>(null);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    async function load() {
      try {
        const [modesResult, cliResult] = await Promise.all([
          commands.listPromptModes(),
          commands.claudeCliAvailable(),
        ]);
        if (modesResult.status === "ok") setModes(modesResult.data);
        if (cliResult.status === "ok") setCliAvailable(cliResult.data);
      } catch {
        // Fora do runtime Tauri (ambiente de teste) `invoke` rejeita — não pode virar uma
        // unhandled rejection.
      }
    }
    void load();
  }, []);

  // Só faz sentido gerar quando existe uma transcrição pronta.
  if (transcriptionPhase !== "done" || transcriptionId === null) return null;

  const selected = modes.find((m) => m.id === mode);

  async function handleCopy() {
    if (!prompt) return;
    await navigator.clipboard.writeText(prompt.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }

  return (
    <div className="flex flex-col gap-3 rounded-lg border border-zinc-800 bg-zinc-900 p-4">
      <div className="flex items-end gap-2">
        <div className="flex flex-1 flex-col gap-1">
          <label htmlFor="prompt-mode" className="text-sm font-medium text-zinc-300">
            Modo do prompt
          </label>
          <select
            id="prompt-mode"
            value={mode}
            onChange={(e) => setMode(e.target.value as PromptMode)}
            className="rounded border border-zinc-700 bg-zinc-950 px-2 py-1.5 text-sm text-zinc-100"
          >
            {modes.map((m) => (
              <option key={m.id} value={m.id}>
                {m.label}
              </option>
            ))}
          </select>
        </div>
        <button
          type="button"
          disabled={generating}
          onClick={() => void generate(transcriptionId)}
          className="rounded bg-zinc-100 px-4 py-1.5 text-sm font-medium text-zinc-900 hover:bg-white disabled:opacity-50"
        >
          {generating ? "Gerando…" : "Gerar prompt"}
        </button>
      </div>

      {selected && <p className="text-xs text-zinc-500">{selected.description}</p>}

      {cliAvailable === false && (
        <p className="text-xs text-amber-400">
          O comando <code>claude</code> não foi encontrado — os prompts serão montados por template
          local (sem reescrita por IA).
        </p>
      )}

      {error && <p className="text-sm text-red-400">{error}</p>}

      {prompt && (
        <div className="flex flex-col gap-2">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-medium text-zinc-300">
              Prompt gerado
              <span className="ml-2 text-xs font-normal text-zinc-500">
                {prompt.generator === "claude_cli" ? "via Claude" : "via template"}
              </span>
            </h3>
            <button
              type="button"
              onClick={() => void handleCopy()}
              className="rounded bg-zinc-800 px-3 py-1 text-xs text-zinc-100 hover:bg-zinc-700"
            >
              {copied ? "Copiado!" : "Copiar"}
            </button>
          </div>

          {fallbackReason && <p className="text-xs text-amber-400">{fallbackReason}</p>}

          <textarea
            readOnly
            value={prompt.content}
            rows={14}
            className="w-full resize-y rounded border border-zinc-700 bg-zinc-950 p-2 font-mono text-xs text-zinc-100"
          />
          <p className="text-xs text-zinc-500">
            Cole no Claude Code. Editar e refinar o prompt chega na Fase 7.
          </p>
        </div>
      )}
    </div>
  );
}
