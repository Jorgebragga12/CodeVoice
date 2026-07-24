import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { commands, type RecordingSettings, type WhisperModelOption } from "../../ipc/bindings";

function formatSize(bytes: number | null): string {
  if (!bytes) return "";
  const gb = bytes / 1_000_000_000;
  if (gb >= 1) return `${gb.toFixed(1)} GB`;
  return `${Math.round(bytes / 1_000_000)} MB`;
}

/**
 * Escolha e download do modelo Whisper (Fase 5). O modelo selecionado é salvo em
 * `settings.whisper_model`; o download roda no backend e reporta progresso por eventos.
 */
export function WhisperModelSettings({
  settings,
  onModelChange,
}: {
  settings: RecordingSettings;
  onModelChange: (modelId: string) => void;
}) {
  const [models, setModels] = useState<WhisperModelOption[]>([]);
  const [downloadingId, setDownloadingId] = useState<string | null>(null);
  const [progress, setProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);

  async function refresh() {
    const result = await commands.listWhisperModels();
    if (result.status === "ok") setModels(result.data);
    else setError(result.error);
  }

  useEffect(() => {
    // Fetch on mount (react.dev/learn/you-might-not-need-an-effect#fetching-data).
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void refresh();
  }, []);

  useEffect(() => {
    // `.catch` porque fora do runtime Tauri (ambiente de teste) `listen` rejeita.
    const unsub = [
      listen<number>("model:download-progress", (e) => setProgress(e.payload)).catch(() => null),
      listen<string>("model:download-done", () => {
        setDownloadingId(null);
        setProgress(0);
        void refresh();
      }).catch(() => null),
      listen<string>("model:download-error", (e) => {
        setDownloadingId(null);
        setError(e.payload);
      }).catch(() => null),
    ];
    return () => {
      for (const p of unsub) void p.then((fn) => fn?.());
    };
  }, []);

  const selected = models.find((m) => m.id === settings.whisper_model);

  async function handleDownload() {
    if (!selected) return;
    setError(null);
    setDownloadingId(selected.id);
    setProgress(0);
    const result = await commands.downloadModel();
    if (result.status === "error") {
      setError(result.error);
      setDownloadingId(null);
    }
  }

  return (
    <section className="flex flex-col gap-2">
      <label htmlFor="whisper-model" className="text-sm font-medium text-zinc-300">
        Modelo de transcrição
      </label>
      <select
        id="whisper-model"
        value={settings.whisper_model}
        onChange={(e) => onModelChange(e.target.value)}
        className="rounded border border-zinc-700 bg-zinc-900 px-2 py-1.5 text-sm text-zinc-100"
      >
        {models.map((m) => (
          <option key={m.id} value={m.id}>
            {m.label}
            {m.downloaded ? " ✓ baixado" : ""}
          </option>
        ))}
      </select>

      {selected && !selected.downloaded && downloadingId === null && (
        <button
          type="button"
          onClick={() => void handleDownload()}
          className="self-start rounded bg-zinc-800 px-3 py-1.5 text-sm text-zinc-100 hover:bg-zinc-700"
        >
          Baixar ({formatSize(selected.size_bytes)})
        </button>
      )}

      {downloadingId !== null && (
        <div className="flex flex-col gap-1">
          <p className="text-xs text-zinc-400">Baixando… {progress}%</p>
          <div className="h-1.5 w-full overflow-hidden rounded-full bg-zinc-800">
            <div className="h-full bg-emerald-500" style={{ width: `${progress}%` }} />
          </div>
        </div>
      )}

      {selected?.downloaded && (
        <p className="text-xs text-emerald-400">Modelo pronto para uso, offline.</p>
      )}

      {error && <p className="text-xs text-red-400">{error}</p>}
    </section>
  );
}
