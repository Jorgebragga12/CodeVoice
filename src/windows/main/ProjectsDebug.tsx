import { useEffect, useState, type FormEvent } from "react";
import { commands, type Project } from "../../ipc/bindings";

/**
 * Tela de verificação da Fase 2 — prova que o pipeline Rust → SQLite → tauri-specta → React
 * funciona de ponta a ponta. Será substituída pela tela real de Projetos na Fase 3.
 */
export function ProjectsDebug() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [name, setName] = useState("");
  const [path, setPath] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  async function loadProjects() {
    setLoading(true);
    try {
      const result = await commands.listProjects();
      if (result.status === "ok") {
        setProjects(result.data);
        setError(null);
      } else {
        setError(result.error);
      }
    } catch {
      setError("Não foi possível falar com o backend do CodeVoice.");
    }
    setLoading(false);
  }

  useEffect(() => {
    // Padrão de "fetch on mount" documentado pelo próprio React (react.dev/learn/you-might-not-need-an-effect#fetching-data);
    // a regra react-hooks/set-state-in-effect ainda não distingue isso de setState direto no corpo do efeito.
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void loadProjects();
  }, []);

  async function handleCreate(event: FormEvent) {
    event.preventDefault();
    if (!name.trim() || !path.trim()) return;

    try {
      const result = await commands.createProject({
        name,
        path,
        description: "",
        stack: "",
        architecture: "",
        dev_commands: "",
        test_commands: "",
        forbidden_tech: "",
        database_info: "",
        notes: "",
      });

      if (result.status === "ok") {
        setName("");
        setPath("");
        await loadProjects();
      } else {
        setError(result.error);
      }
    } catch {
      setError("Não foi possível falar com o backend do CodeVoice.");
    }
  }

  return (
    <div className="w-full max-w-md rounded-lg border border-zinc-800 bg-zinc-900 p-4 text-left text-sm">
      <h2 className="mb-2 font-medium text-zinc-300">Debug — Projetos (Fase 2)</h2>

      {error && <p className="mb-2 text-red-400">{error}</p>}

      {loading ? (
        <p className="text-zinc-500">Carregando…</p>
      ) : projects.length === 0 ? (
        <p className="mb-3 text-zinc-500">Nenhum projeto cadastrado ainda.</p>
      ) : (
        <ul className="mb-3 space-y-1">
          {projects.map((p) => (
            <li key={p.id} className="text-zinc-400">
              <span className="text-zinc-200">{p.name}</span> — {p.path}
            </li>
          ))}
        </ul>
      )}

      <form onSubmit={handleCreate} className="flex flex-col gap-2">
        <input
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="Nome do projeto"
          className="rounded border border-zinc-700 bg-zinc-950 px-2 py-1 text-zinc-100 placeholder:text-zinc-600"
        />
        <input
          value={path}
          onChange={(e) => setPath(e.target.value)}
          placeholder="Caminho local"
          className="rounded border border-zinc-700 bg-zinc-950 px-2 py-1 text-zinc-100 placeholder:text-zinc-600"
        />
        <button
          type="submit"
          className="rounded bg-zinc-800 px-2 py-1 text-zinc-200 hover:bg-zinc-700"
        >
          Adicionar
        </button>
      </form>
    </div>
  );
}
