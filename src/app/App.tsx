import { ErrorBoundary } from "./ErrorBoundary";
import { MainWindow } from "../windows/main/MainWindow";
import { Recorder } from "../windows/recorder/Recorder";

/**
 * Ambas as janelas (principal e recorder) carregam a mesma `index.html`; o query param
 * `?window=recorder` decide qual árvore React renderizar. É o padrão do Tauri para múltiplas
 * janelas sem múltiplos entrypoints de build.
 */
export function App() {
  const params = new URLSearchParams(window.location.search);
  const isRecorder = params.get("window") === "recorder";

  return <ErrorBoundary>{isRecorder ? <Recorder /> : <MainWindow />}</ErrorBoundary>;
}
