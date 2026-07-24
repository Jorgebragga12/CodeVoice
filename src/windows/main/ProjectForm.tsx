import { useState, type FormEvent } from "react";
import { commands, type ImportPreview, type Project, type ProjectUpdate } from "../../ipc/bindings";
import { useProjectStore } from "../../stores/projectStore";
import { ImportPreviewPanel } from "./ImportPreviewPanel";
import { ProjectContextFields } from "./ProjectContextFields";
import { ProjectRulesEditor } from "./ProjectRulesEditor";

interface ProjectFormProps {
  /** Ausente = modo criação; presente = modo edição (o `path` deixa de ser editável). */
  project?: Project;
  onSaved: () => void;
  onCancel: () => void;
}

const emptyFields = {
  name: "",
  path: "",
  description: "",
  stack: "",
  architecture: "",
  dev_commands: "",
  test_commands: "",
  forbidden_tech: "",
  database_info: "",
  notes: "",
};

type ProjectFormFields = typeof emptyFields;

function toFormFields(project?: Project): ProjectFormFields {
  if (!project) return emptyFields;
  return {
    name: project.name,
    path: project.path,
    description: project.description,
    stack: project.stack,
    architecture: project.architecture,
    dev_commands: project.dev_commands,
    test_commands: project.test_commands,
    forbidden_tech: project.forbidden_tech,
    database_info: project.database_info,
    notes: project.notes,
  };
}

type PathStatus = "idle" | "checking" | "valid" | "invalid";

export function ProjectForm({ project, onSaved, onCancel }: ProjectFormProps) {
  const isEdit = project !== undefined;
  const create = useProjectStore((s) => s.create);
  const update = useProjectStore((s) => s.update);

  const [fields, setFields] = useState<ProjectFormFields>(() => toFormFields(project));
  const [saving, setSaving] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);

  const [pathStatus, setPathStatus] = useState<PathStatus>("idle");
  const [pathError, setPathError] = useState<string | null>(null);
  const [previewLoading, setPreviewLoading] = useState(false);
  const [preview, setPreview] = useState<ImportPreview | null>(null);
  const [previewError, setPreviewError] = useState<string | null>(null);

  function setField(key: keyof ProjectFormFields, value: string) {
    setFields((prev) => ({ ...prev, [key]: value }));
  }

  async function checkPath() {
    if (!fields.path.trim()) {
      setPathStatus("idle");
      return;
    }
    setPathStatus("checking");
    try {
      const result = await commands.validateProjectPath(fields.path);
      if (result.status === "ok") {
        setPathStatus("valid");
        setPathError(null);
      } else {
        setPathStatus("invalid");
        setPathError(result.error);
      }
    } catch {
      setPathStatus("invalid");
      setPathError("Não foi possível validar o caminho.");
    }
  }

  async function handlePreviewImport() {
    setPreviewError(null);
    setPreview(null);
    if (!fields.path.trim()) {
      setPreviewError("Informe o caminho do projeto antes de pré-visualizar a importação.");
      return;
    }

    setPreviewLoading(true);
    try {
      const result = await commands.previewProjectImport(fields.path);
      if (result.status === "ok") {
        setPreview(result.data);
      } else {
        setPreviewError(result.error);
      }
    } catch {
      setPreviewError("Não foi possível ler o diretório do projeto.");
    }
    setPreviewLoading(false);
  }

  function handleUseImportedContent(digest: string) {
    setField("notes", fields.notes ? `${fields.notes}\n\n${digest}` : digest);
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    if (!fields.name.trim() || (!isEdit && !fields.path.trim())) {
      setFormError("Nome e caminho são obrigatórios.");
      return;
    }

    setSaving(true);
    setFormError(null);

    const updatePayload: ProjectUpdate = {
      name: fields.name,
      description: fields.description,
      stack: fields.stack,
      architecture: fields.architecture,
      dev_commands: fields.dev_commands,
      test_commands: fields.test_commands,
      forbidden_tech: fields.forbidden_tech,
      database_info: fields.database_info,
      notes: fields.notes,
    };

    const result =
      isEdit && project
        ? await update(project.id, updatePayload)
        : await create({ ...updatePayload, path: fields.path });

    setSaving(false);

    if (result) {
      onSaved();
    } else {
      setFormError(useProjectStore.getState().error ?? "Não foi possível salvar o projeto.");
    }
  }

  return (
    <div className="w-full max-w-2xl rounded-lg border border-zinc-800 bg-zinc-900 p-5">
      <h2 className="mb-4 text-lg font-semibold text-zinc-100">
        {isEdit ? `Editar ${project.name}` : "Novo projeto"}
      </h2>

      <form onSubmit={handleSubmit} className="flex flex-col gap-3 text-sm">
        <label className="flex flex-col gap-1">
          <span className="text-zinc-400">Nome</span>
          <input
            value={fields.name}
            onChange={(e) => setField("name", e.target.value)}
            className="rounded border border-zinc-700 bg-zinc-950 px-2 py-1 text-zinc-100"
            required
          />
        </label>

        <label className="flex flex-col gap-1">
          <span className="text-zinc-400">Caminho local</span>
          {isEdit ? (
            <input
              value={fields.path}
              disabled
              className="rounded border border-zinc-800 bg-zinc-950 px-2 py-1 text-zinc-500"
            />
          ) : (
            <>
              <div className="flex gap-2">
                <input
                  value={fields.path}
                  onChange={(e) => {
                    setField("path", e.target.value);
                    setPathStatus("idle");
                  }}
                  onBlur={() => void checkPath()}
                  placeholder="C:\projects\meu-projeto"
                  className="flex-1 rounded border border-zinc-700 bg-zinc-950 px-2 py-1 text-zinc-100"
                  required
                />
                <button
                  type="button"
                  onClick={() => void handlePreviewImport()}
                  disabled={previewLoading}
                  className="shrink-0 rounded bg-zinc-800 px-3 py-1 text-zinc-200 hover:bg-zinc-700 disabled:opacity-50"
                >
                  {previewLoading ? "Lendo…" : "Pré-visualizar importação"}
                </button>
              </div>
              {pathStatus === "checking" && (
                <p className="text-xs text-zinc-500">Verificando caminho…</p>
              )}
              {pathStatus === "valid" && (
                <p className="text-xs text-emerald-400">Caminho válido.</p>
              )}
              {pathStatus === "invalid" && pathError && (
                <p className="text-xs text-red-400">{pathError}</p>
              )}
            </>
          )}
        </label>

        {previewError && <p className="text-xs text-red-400">{previewError}</p>}
        {preview && (
          <ImportPreviewPanel preview={preview} onUseContent={handleUseImportedContent} />
        )}

        <label className="flex flex-col gap-1">
          <span className="text-zinc-400">Descrição</span>
          <textarea
            value={fields.description}
            onChange={(e) => setField("description", e.target.value)}
            rows={2}
            className="rounded border border-zinc-700 bg-zinc-950 px-2 py-1 text-zinc-100"
          />
        </label>

        <ProjectContextFields value={fields} onChange={setField} />

        <label className="flex flex-col gap-1">
          <span className="text-zinc-400">Notas</span>
          <textarea
            value={fields.notes}
            onChange={(e) => setField("notes", e.target.value)}
            rows={4}
            className="rounded border border-zinc-700 bg-zinc-950 px-2 py-1 font-mono text-xs text-zinc-100"
          />
        </label>

        {formError && <p className="text-sm text-red-400">{formError}</p>}

        <div className="mt-2 flex justify-end gap-2">
          <button
            type="button"
            onClick={onCancel}
            className="rounded px-3 py-1.5 text-zinc-300 hover:bg-zinc-800"
          >
            Cancelar
          </button>
          <button
            type="submit"
            disabled={saving}
            className="rounded bg-zinc-100 px-3 py-1.5 text-zinc-900 hover:bg-white disabled:opacity-50"
          >
            {saving ? "Salvando…" : "Salvar projeto"}
          </button>
        </div>
      </form>

      {isEdit && project && (
        <div className="mt-5">
          <ProjectRulesEditor projectId={project.id} />
        </div>
      )}
    </div>
  );
}
