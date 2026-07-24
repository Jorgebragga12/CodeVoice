import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { commands, type Recording } from "../../ipc/bindings";
import { useProjectStore } from "../../stores/projectStore";
import { useTranscriptionStore } from "../../stores/transcriptionStore";
import { formatDuration } from "../../lib/format";

/**
 * Faixa do fluxo principal (PRODUCT-SPEC §3): escolher projeto ativo e iniciar/parar a gravação
 * pela UI. O atalho global faz o mesmo sem depender desta tela estar visível.
 */
export function RecordBar() {
  const projects = useProjectStore((s) => s.projects);
  const load = useProjectStore((s) => s.load);

  const startTranscription = useTranscriptionStore((s) => s.start);

  const [activeProject, setActiveProject] = useState<number | null>(null);
  const [isRecording, setIsRecording] = useState(false);
  const [elapsedMs, setElapsedMs] = useState(0);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    void load();
  }, [load]);

  // Dispara a transcrição sempre que uma gravação encerra, seja qual for o caminho (botão,
  // atalho global, janela recorder, limite de 10 min) — todos emitem `recording:stopped`.
  useEffect(() => {
    // `.catch` porque fora do runtime Tauri (ex.: ambiente de teste) `listen` rejeita — não
    // pode virar uma unhandled rejection.
    const unlisten = listen<Recording>("recording:stopped", (e) => {
      void startTranscription(e.payload.id);
    }).catch(() => null);
    return () => {
      void unlisten.then((fn) => fn?.());
    };
  }, [startTranscription]);

  useEffect(() => {
    let active = true;
    async function poll() {
      try {
        const result = await commands.recordingStatus();
        if (!active || result.status !== "ok") return;
        setIsRecording(result.data.state === "recording");
        setElapsedMs(result.data.elapsed_ms);
      } catch {
        // ignora: backend pode estar reiniciando
      }
    }
    void poll();
    const timer = setInterval(() => void poll(), 250);
    return () => {
      active = false;
      clearInterval(timer);
    };
  }, []);

  async function chooseProject(id: number | null) {
    setActiveProject(id);
    // Informa o backend para que o atalho global saiba a que projeto associar a gravação
    // mesmo com esta janela minimizada.
    await commands.setActiveProject(id);
  }

  async function toggle() {
    setError(null);
    if (isRecording) {
      const result = await commands.stopRecording();
      if (result.status === "error") setError(result.error);
      // Não dispara a transcrição aqui: o backend emite `recording:stopped` em TODOS os
      // caminhos de encerramento (botão, atalho global, janela recorder, limite de 10 min), e
      // o listener abaixo cuida disso de um lugar só.
      return;
    }
    const shown = await commands.showRecorderWindow();
    if (shown.status === "error") {
      setError(shown.error);
      return;
    }
    const result = await commands.startRecording();
    if (result.status === "error") setError(result.error);
  }

  return (
    <div className="flex flex-col gap-2 rounded-lg border border-zinc-800 bg-zinc-900 p-4">
      <div className="flex items-center gap-3">
        <select
          value={activeProject ?? ""}
          onChange={(e) => void chooseProject(e.target.value ? Number(e.target.value) : null)}
          className="flex-1 rounded border border-zinc-700 bg-zinc-950 px-2 py-1.5 text-sm text-zinc-100"
        >
          <option value="">Sem projeto</option>
          {projects.map((p) => (
            <option key={p.id} value={p.id}>
              {p.name}
            </option>
          ))}
        </select>

        {isRecording && (
          <span className="flex items-center gap-1.5 font-mono text-sm tabular-nums text-zinc-100">
            <span aria-hidden className="h-2 w-2 animate-pulse rounded-full bg-red-500" />
            {formatDuration(elapsedMs)}
          </span>
        )}

        <button
          type="button"
          onClick={() => void toggle()}
          className={`rounded px-4 py-1.5 text-sm font-medium ${
            isRecording
              ? "bg-red-600 text-white hover:bg-red-500"
              : "bg-zinc-100 text-zinc-900 hover:bg-white"
          }`}
        >
          {isRecording ? "Parar" : "Gravar"}
        </button>
      </div>

      {error && <p className="text-sm text-red-400">{error}</p>}
    </div>
  );
}
