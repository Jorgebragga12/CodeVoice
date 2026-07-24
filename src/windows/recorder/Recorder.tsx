import { useEffect, useState } from "react";
import { commands } from "../../ipc/bindings";
import { formatDuration } from "../../lib/format";

/**
 * Janela compacta de gravação: sem borda, sempre no topo. Mostra estado, contador e projeto
 * ativo. `Esc` cancela (PRODUCT-SPEC §5.2).
 *
 * O contador vem de polling do backend (`recording_status`) e não de um cronômetro local: o
 * backend é a fonte da verdade do tempo, então a janela mostra a duração real mesmo se o
 * webview travar por um instante ou se a gravação tiver sido encerrada pelo limite de 10 min.
 */
export function Recorder({ projectName }: { projectName?: string }) {
  const [elapsedMs, setElapsedMs] = useState(0);
  const [isRecording, setIsRecording] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let active = true;

    async function poll() {
      try {
        const result = await commands.recordingStatus();
        if (!active) return;
        if (result.status === "ok") {
          setElapsedMs(result.data.elapsed_ms);
          setIsRecording(result.data.state === "recording");
        } else {
          setError(result.error);
        }
      } catch {
        // Backend indisponível (ex.: durante o encerramento do app) — nada a fazer aqui.
      }
    }

    void poll();
    const timer = setInterval(() => void poll(), 200);
    return () => {
      active = false;
      clearInterval(timer);
    };
  }, []);

  useEffect(() => {
    async function onKeyDown(event: KeyboardEvent) {
      if (event.key !== "Escape") return;
      try {
        await commands.cancelRecording();
      } catch {
        // Mesmo se o cancelamento falhar, esconder a janela é o comportamento esperado.
      }
      await commands.hideRecorderWindow();
    }

    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, []);

  async function handleStop() {
    const result = await commands.stopRecording();
    if (result.status === "error") {
      setError(result.error);
      return;
    }
    await commands.hideRecorderWindow();
  }

  async function handleCancel() {
    await commands.cancelRecording();
    await commands.hideRecorderWindow();
  }

  return (
    <div
      data-tauri-drag-region
      className="flex h-screen w-screen select-none flex-col justify-between rounded-lg border border-zinc-800 bg-zinc-950 p-3"
    >
      <div className="flex items-center gap-2" data-tauri-drag-region>
        <span
          aria-hidden
          className={`h-2.5 w-2.5 rounded-full ${
            isRecording ? "animate-pulse bg-red-500" : "bg-zinc-600"
          }`}
        />
        <span className="text-xs font-medium text-zinc-300">
          {isRecording ? "Gravando" : "Parado"}
        </span>
        <span className="ml-auto font-mono text-sm tabular-nums text-zinc-100">
          {formatDuration(elapsedMs)}
        </span>
      </div>

      <p className="truncate text-[11px] text-zinc-500" data-tauri-drag-region>
        {projectName ? `Projeto: ${projectName}` : "Nenhum projeto selecionado"}
      </p>

      {error && <p className="truncate text-[11px] text-red-400">{error}</p>}

      <div className="flex gap-2">
        <button
          type="button"
          onClick={() => void handleStop()}
          className="flex-1 rounded bg-zinc-800 px-2 py-1 text-xs text-zinc-100 hover:bg-zinc-700"
        >
          Parar
        </button>
        <button
          type="button"
          onClick={() => void handleCancel()}
          className="rounded px-2 py-1 text-xs text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200"
        >
          Cancelar (Esc)
        </button>
      </div>
    </div>
  );
}
