import { useEffect, useState } from "react";
import { ConfirmDialog } from "../../components/ConfirmDialog";
import type { Project } from "../../ipc/bindings";
import { useProjectStore } from "../../stores/projectStore";
import { ProjectForm } from "./ProjectForm";

/**
 * Tela real de Projetos (Fase 3): lista, cria, edita e exclui (com confirmação) — substitui
 * `ProjectsDebug.tsx` da Fase 2.
 */
export function Projects() {
  const projects = useProjectStore((s) => s.projects);
  const loading = useProjectStore((s) => s.loading);
  const error = useProjectStore((s) => s.error);
  const load = useProjectStore((s) => s.load);
  const remove = useProjectStore((s) => s.remove);

  const [formOpen, setFormOpen] = useState(false);
  const [editingProject, setEditingProject] = useState<Project | undefined>(undefined);
  const [deletingProject, setDeletingProject] = useState<Project | null>(null);
  const [deleting, setDeleting] = useState(false);

  useEffect(() => {
    void load();
  }, [load]);

  function openCreate() {
    setEditingProject(undefined);
    setFormOpen(true);
  }

  function openEdit(project: Project) {
    setEditingProject(project);
    setFormOpen(true);
  }

  function closeForm() {
    setFormOpen(false);
    setEditingProject(undefined);
  }

  async function confirmDelete() {
    if (!deletingProject) return;
    setDeleting(true);
    await remove(deletingProject.id);
    setDeleting(false);
    setDeletingProject(null);
  }

  return (
    <div className="w-full max-w-3xl">
      <div className="mb-3 flex items-center justify-between">
        <h2 className="text-lg font-semibold text-zinc-100">Projetos</h2>
        <button
          type="button"
          onClick={openCreate}
          className="rounded bg-zinc-100 px-3 py-1.5 text-sm font-medium text-zinc-900 hover:bg-white"
        >
          + Novo projeto
        </button>
      </div>

      {error && <p className="mb-2 text-sm text-red-400">{error}</p>}

      {loading ? (
        <p className="text-sm text-zinc-500">Carregando…</p>
      ) : projects.length === 0 ? (
        <p className="rounded-lg border border-dashed border-zinc-800 p-6 text-center text-sm text-zinc-500">
          Nenhum projeto cadastrado ainda. Clique em "+ Novo projeto" para começar.
        </p>
      ) : (
        <ul className="space-y-2">
          {projects.map((project) => (
            <li
              key={project.id}
              className="flex items-center justify-between gap-3 rounded-lg border border-zinc-800 bg-zinc-900 px-4 py-3"
            >
              <div className="min-w-0">
                <p className="truncate font-medium text-zinc-100">{project.name}</p>
                <p className="truncate text-xs text-zinc-500">{project.path}</p>
              </div>
              <div className="flex shrink-0 gap-1">
                <button
                  type="button"
                  onClick={() => openEdit(project)}
                  className="rounded px-2 py-1 text-sm text-zinc-300 hover:bg-zinc-800"
                >
                  Editar
                </button>
                <button
                  type="button"
                  onClick={() => setDeletingProject(project)}
                  className="rounded px-2 py-1 text-sm text-red-400 hover:bg-red-950/40"
                >
                  Excluir
                </button>
              </div>
            </li>
          ))}
        </ul>
      )}

      {formOpen && (
        <div className="fixed inset-0 z-40 flex items-center justify-center overflow-y-auto bg-black/60 p-4">
          <ProjectForm project={editingProject} onSaved={closeForm} onCancel={closeForm} />
        </div>
      )}

      <ConfirmDialog
        open={deletingProject !== null}
        title="Excluir projeto"
        message={
          deletingProject
            ? `Tem certeza que deseja excluir "${deletingProject.name}"? O histórico relacionado é mantido, mas deixa de referenciar este projeto. Esta ação não pode ser desfeita.`
            : ""
        }
        confirmLabel={deleting ? "Excluindo…" : "Excluir projeto"}
        danger
        busy={deleting}
        onConfirm={() => void confirmDelete()}
        onCancel={() => setDeletingProject(null)}
      />
    </div>
  );
}
