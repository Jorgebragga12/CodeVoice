import { create } from "zustand";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { commands } from "../ipc/bindings";

/**
 * Fases do fluxo de transcrição:
 * - `idle`: nada em andamento.
 * - `checking`: consultando se o modelo está pronto.
 * - `need_download`: modelo ausente; a UI oferece baixar.
 * - `downloading`: baixando o modelo (progress = %).
 * - `transcribing`: rodando o Whisper (progress = %).
 * - `done`: `text` preenchido.
 * - `error`: `error` preenchido.
 */
export type TranscriptionPhase =
  "idle" | "checking" | "need_download" | "downloading" | "transcribing" | "done" | "error";

interface TranscriptionState {
  phase: TranscriptionPhase;
  progress: number;
  text: string | null;
  error: string | null;
  recordingId: number | null;

  /** Inicia o fluxo para uma gravação recém-encerrada. */
  start: (recordingId: number) => Promise<void>;
  /** Baixa o modelo e, ao terminar, transcreve automaticamente. */
  downloadThenTranscribe: () => Promise<void>;
  setText: (text: string) => void;
  reset: () => void;
}

async function runTranscription(
  recordingId: number,
  set: (partial: Partial<TranscriptionState>) => void,
) {
  set({ phase: "transcribing", progress: 0, error: null });

  let unlisten: UnlistenFn | undefined;
  try {
    unlisten = await listen<number>("transcription:progress", (e) => {
      set({ progress: e.payload });
    });

    const result = await commands.transcribeRecording(recordingId);
    if (result.status === "ok") {
      set({ phase: "done", progress: 100, text: result.data.text });
    } else {
      set({ phase: "error", error: result.error });
    }
  } catch (e) {
    set({ phase: "error", error: e instanceof Error ? e.message : String(e) });
  } finally {
    unlisten?.();
  }
}

export const useTranscriptionStore = create<TranscriptionState>((set, get) => ({
  phase: "idle",
  progress: 0,
  text: null,
  error: null,
  recordingId: null,

  async start(recordingId: number) {
    set({
      phase: "checking",
      progress: 0,
      text: null,
      error: null,
      recordingId,
    });

    const status = await commands.transcriptionStatus();
    if (status.status === "error") {
      set({ phase: "error", error: status.error });
      return;
    }

    if (status.data.status === "ready") {
      await runTranscription(recordingId, set);
    } else {
      // Modelo ausente (ou baixando de outra origem): a UI oferece baixar.
      set({ phase: "need_download" });
    }
  },

  async downloadThenTranscribe() {
    const recordingId = get().recordingId;
    if (recordingId === null) return;

    set({ phase: "downloading", progress: 0, error: null });

    let unlistenProgress: UnlistenFn | undefined;
    let unlistenDone: UnlistenFn | undefined;
    let unlistenError: UnlistenFn | undefined;

    const cleanup = () => {
      unlistenProgress?.();
      unlistenDone?.();
      unlistenError?.();
    };

    try {
      unlistenProgress = await listen<number>("model:download-progress", (e) => {
        set({ progress: e.payload });
      });
      unlistenDone = await listen<string>("model:download-done", () => {
        cleanup();
        void runTranscription(recordingId, set);
      });
      unlistenError = await listen<string>("model:download-error", (e) => {
        cleanup();
        set({ phase: "error", error: e.payload });
      });

      const started = await commands.downloadModel();
      if (started.status === "error") {
        cleanup();
        set({ phase: "error", error: started.error });
      }
    } catch (e) {
      cleanup();
      set({ phase: "error", error: e instanceof Error ? e.message : String(e) });
    }
  },

  setText(text: string) {
    set({ text });
  },

  reset() {
    set({ phase: "idle", progress: 0, text: null, error: null, recordingId: null });
  },
}));
