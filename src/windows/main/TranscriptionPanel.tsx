import { useTranscriptionStore } from "../../stores/transcriptionStore";

/**
 * Mostra o andamento e o resultado da transcrição (Fase 5). A edição do texto e a geração do
 * prompt técnico vêm nas Fases 6–7; aqui o texto é exibido em somente-leitura.
 */
export function TranscriptionPanel() {
  const phase = useTranscriptionStore((s) => s.phase);
  const progress = useTranscriptionStore((s) => s.progress);
  const text = useTranscriptionStore((s) => s.text);
  const error = useTranscriptionStore((s) => s.error);
  const download = useTranscriptionStore((s) => s.downloadThenTranscribe);
  const reset = useTranscriptionStore((s) => s.reset);

  if (phase === "idle") return null;

  return (
    <div className="rounded-lg border border-zinc-800 bg-zinc-900 p-4">
      {phase === "checking" && <p className="text-sm text-zinc-400">Verificando o modelo…</p>}

      {phase === "need_download" && (
        <div className="flex flex-col gap-3">
          <p className="text-sm text-zinc-300">
            O modelo de transcrição ainda não foi baixado. Ele fica salvo no seu computador e é
            usado offline.
          </p>
          <div className="flex gap-2">
            <button
              type="button"
              onClick={() => void download()}
              className="rounded bg-zinc-100 px-3 py-1.5 text-sm font-medium text-zinc-900 hover:bg-white"
            >
              Baixar modelo agora
            </button>
            <button
              type="button"
              onClick={reset}
              className="rounded px-3 py-1.5 text-sm text-zinc-400 hover:text-zinc-200"
            >
              Cancelar
            </button>
          </div>
        </div>
      )}

      {phase === "downloading" && (
        <div className="flex flex-col gap-2">
          <p className="text-sm text-zinc-300">Baixando o modelo… {progress}%</p>
          <div className="h-2 w-full overflow-hidden rounded-full bg-zinc-800">
            <div
              className="h-full bg-emerald-500 transition-[width] duration-200"
              style={{ width: `${progress}%` }}
            />
          </div>
          <p className="text-xs text-zinc-500">
            O download acontece só uma vez; nas próximas gravações a transcrição é imediata.
          </p>
        </div>
      )}

      {phase === "transcribing" && (
        <div className="flex flex-col gap-2">
          {/* Barra indeterminada: o Whisper não dá % confiável em clipes curtos, então mostramos
              atividade em vez de um número que fica parado. */}
          <p className="text-sm text-zinc-300">Transcrevendo…</p>
          <div className="relative h-2 w-full overflow-hidden rounded-full bg-zinc-800 indeterminate-bar" />
        </div>
      )}

      {phase === "done" && (
        <div className="flex flex-col gap-2">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-medium text-zinc-300">Transcrição</h3>
            <button
              type="button"
              onClick={reset}
              className="text-xs text-zinc-500 hover:text-zinc-300"
            >
              Limpar
            </button>
          </div>
          <textarea
            readOnly
            value={text ?? ""}
            rows={5}
            className="w-full resize-y rounded border border-zinc-700 bg-zinc-950 p-2 text-sm text-zinc-100"
          />
          <p className="text-xs text-zinc-500">
            Próximo passo (Fase 6): transformar isto num prompt técnico para o Claude Code.
          </p>
        </div>
      )}

      {phase === "error" && (
        <div className="flex flex-col gap-2">
          <p className="text-sm text-red-400">{error}</p>
          <button
            type="button"
            onClick={reset}
            className="self-start rounded px-3 py-1.5 text-sm text-zinc-400 hover:text-zinc-200"
          >
            Fechar
          </button>
        </div>
      )}
    </div>
  );
}
