import { useEffect, useState, type FormEvent } from "react";
import { commands, type ProjectRule } from "../../ipc/bindings";

interface ProjectRulesEditorProps {
  projectId: number;
}

const UNREACHABLE_ERROR = "Não foi possível falar com o backend do CodeVoice.";

/**
 * CRUD de `project_rules` com `sort_order` (Fase 3). Só existe para projetos já salvos —
 * precisa de um `projectId` real pra associar as regras.
 */
export function ProjectRulesEditor({ projectId }: ProjectRulesEditorProps) {
  const [rules, setRules] = useState<ProjectRule[]>([]);
  const [newRule, setNewRule] = useState("");
  const [error, setError] = useState<string | null>(null);

  async function load() {
    try {
      const result = await commands.listProjectRules(projectId);
      if (result.status === "ok") {
        setRules(result.data);
      } else {
        setError(result.error);
      }
    } catch {
      setError(UNREACHABLE_ERROR);
    }
  }

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void load();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [projectId]);

  async function handleAdd(event: FormEvent) {
    event.preventDefault();
    if (!newRule.trim()) return;

    try {
      const result = await commands.createProjectRule({
        project_id: projectId,
        rule: newRule.trim(),
      });
      if (result.status === "ok") {
        setNewRule("");
        await load();
      } else {
        setError(result.error);
      }
    } catch {
      setError(UNREACHABLE_ERROR);
    }
  }

  async function handleDelete(id: number) {
    try {
      const result = await commands.deleteProjectRule(id);
      if (result.status === "ok") {
        await load();
      } else {
        setError(result.error);
      }
    } catch {
      setError(UNREACHABLE_ERROR);
    }
  }

  async function handleMove(index: number, direction: -1 | 1) {
    const target = index + direction;
    if (target < 0 || target >= rules.length) return;

    const reordered = [...rules];
    const [moved] = reordered.splice(index, 1);
    reordered.splice(target, 0, moved);
    setRules(reordered);

    try {
      const result = await commands.reorderProjectRules(
        projectId,
        reordered.map((rule) => rule.id),
      );
      if (result.status !== "ok") {
        setError(result.error);
        await load();
      }
    } catch {
      setError(UNREACHABLE_ERROR);
      await load();
    }
  }

  return (
    <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3">
      <h3 className="mb-2 text-sm font-medium text-zinc-300">Regras do projeto</h3>
      {error && <p className="mb-2 text-xs text-red-400">{error}</p>}

      {rules.length === 0 ? (
        <p className="mb-2 text-xs text-zinc-500">Nenhuma regra cadastrada.</p>
      ) : (
        <ul className="mb-2 space-y-1">
          {rules.map((rule, index) => (
            <li key={rule.id} className="flex items-center gap-2 text-sm text-zinc-300">
              <div className="flex flex-col leading-none">
                <button
                  type="button"
                  disabled={index === 0}
                  onClick={() => void handleMove(index, -1)}
                  className="text-xs text-zinc-500 hover:text-zinc-200 disabled:opacity-30"
                  aria-label={`Mover "${rule.rule}" para cima`}
                >
                  ▲
                </button>
                <button
                  type="button"
                  disabled={index === rules.length - 1}
                  onClick={() => void handleMove(index, 1)}
                  className="text-xs text-zinc-500 hover:text-zinc-200 disabled:opacity-30"
                  aria-label={`Mover "${rule.rule}" para baixo`}
                >
                  ▼
                </button>
              </div>
              <span className="flex-1">{rule.rule}</span>
              <button
                type="button"
                onClick={() => void handleDelete(rule.id)}
                className="text-xs text-red-400 hover:text-red-300"
              >
                Remover
              </button>
            </li>
          ))}
        </ul>
      )}

      <form onSubmit={handleAdd} className="flex gap-2">
        <input
          value={newRule}
          onChange={(e) => setNewRule(e.target.value)}
          placeholder="Nova regra (ex.: nunca usar i64 em IDs)"
          className="flex-1 rounded border border-zinc-700 bg-zinc-900 px-2 py-1 text-sm text-zinc-100 placeholder:text-zinc-600"
        />
        <button
          type="submit"
          className="rounded bg-zinc-800 px-2 py-1 text-sm text-zinc-200 hover:bg-zinc-700"
        >
          Adicionar
        </button>
      </form>
    </div>
  );
}
