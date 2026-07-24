import { Projects } from "./Projects";

export function Home() {
  return (
    <div className="flex h-full flex-col items-center gap-6 overflow-y-auto p-6">
      <div className="flex flex-col items-center gap-2">
        <h1 className="text-2xl font-semibold tracking-tight text-zinc-100">CodeVoice</h1>
        <p className="text-sm text-zinc-500">Fala vira prompt técnico para o Claude Code.</p>
      </div>
      <Projects />
    </div>
  );
}
