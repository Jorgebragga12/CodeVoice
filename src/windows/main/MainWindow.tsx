import { useState } from "react";
import { Projects } from "./Projects";
import { RecordBar } from "./RecordBar";
import { Settings } from "./Settings";
import { TranscriptionPanel } from "./TranscriptionPanel";

type Tab = "projects" | "settings";

export function MainWindow() {
  const [tab, setTab] = useState<Tab>("projects");

  return (
    <div className="flex h-screen flex-col bg-zinc-950 text-zinc-100">
      <header className="flex items-center gap-4 border-b border-zinc-800 px-6 py-3">
        <h1 className="text-lg font-semibold tracking-tight">CodeVoice</h1>
        <nav className="ml-auto flex gap-1 text-sm">
          <TabButton active={tab === "projects"} onClick={() => setTab("projects")}>
            Projetos
          </TabButton>
          <TabButton active={tab === "settings"} onClick={() => setTab("settings")}>
            Configurações
          </TabButton>
        </nav>
      </header>

      <main className="flex-1 overflow-y-auto p-6">
        <div className="mx-auto flex max-w-3xl flex-col gap-6">
          <RecordBar />
          <TranscriptionPanel />
          {tab === "projects" ? <Projects /> : <Settings />}
        </div>
      </main>
    </div>
  );
}

function TabButton({
  active,
  onClick,
  children,
}: {
  active: boolean;
  onClick: () => void;
  children: React.ReactNode;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={`rounded px-3 py-1 ${
        active ? "bg-zinc-800 text-zinc-100" : "text-zinc-400 hover:text-zinc-200"
      }`}
    >
      {children}
    </button>
  );
}
