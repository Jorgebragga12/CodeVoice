import { useEffect, useState } from "react";
import { commands, type PromptTemplate, type TemplateCategory } from "../../ipc/bindings";
import { ConfirmDialog } from "../../components/ConfirmDialog";
import { usePromptStore } from "../../stores/promptStore";
import { useTranscriptionStore } from "../../stores/transcriptionStore";

/** Fora do runtime Tauri (ambiente de teste) `invoke` rejeita — vira lista vazia, não crash. */
async function fetchCategories(): Promise<TemplateCategory[] | null> {
  try {
    const result = await commands.listTemplateCategories();
    return result.status === "ok" ? result.data : null;
  } catch {
    return null;
  }
}

/**
 * Biblioteca de modelos: os 117 embutidos (agrupados pelas 18 categorias de `templates/`) mais
 * os que o usuário salvou pelo editor. "Usar este modelo" gera um prompt encaixando a
 * transcrição atual no lugar do marcador `<<SUA FALA>>`.
 */
export function TemplateLibrary() {
  const transcriptionId = useTranscriptionStore((s) => s.transcriptionId);
  const generateFromTemplate = usePromptStore((s) => s.generateFromTemplate);
  const generating = usePromptStore((s) => s.generating);

  const [categories, setCategories] = useState<TemplateCategory[]>([]);
  const [active, setActive] = useState<string | null>(null);
  const [items, setItems] = useState<PromptTemplate[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [pendingDelete, setPendingDelete] = useState<PromptTemplate | null>(null);

  useEffect(() => {
    async function load() {
      const data = await fetchCategories();
      if (!data) return;
      setCategories(data);
      setActive((current) => current ?? data[0]?.id ?? null);
    }
    void load();
  }, []);

  useEffect(() => {
    if (active === null) return;
    async function load(category: string) {
      try {
        const result = await commands.listPromptTemplates(category);
        if (result.status === "ok") setItems(result.data);
        else setError(result.error);
      } catch {
        // idem
      }
    }
    void load(active);
  }, [active]);

  async function handleDelete() {
    if (!pendingDelete) return;
    const result = await commands.deletePromptTemplate(pendingDelete.id);
    if (result.status === "error") {
      setError(result.error);
    } else {
      setItems((current) => current.filter((t) => t.id !== pendingDelete.id));
      // A contagem da categoria mudou; sem recarregar, o número ao lado do rótulo mente.
      const data = await fetchCategories();
      if (data) setCategories(data);
    }
    setPendingDelete(null);
  }

  return (
    <section className="flex flex-col gap-4">
      <div>
        <h2 className="text-base font-semibold text-zinc-100">Modelos</h2>
        <p className="text-xs text-zinc-500">
          {transcriptionId === null
            ? "Grave e transcreva algo para poder usar um modelo."
            : "Escolha um modelo para transformar a transcrição atual num prompt."}
        </p>
      </div>

      {error && <p className="text-sm text-red-400">{error}</p>}

      <div className="flex flex-wrap gap-1.5">
        {categories.map((category) => (
          <button
            key={category.id}
            type="button"
            onClick={() => setActive(category.id)}
            className={`rounded px-2.5 py-1 text-xs ${
              active === category.id
                ? "bg-zinc-100 text-zinc-900"
                : "bg-zinc-800 text-zinc-300 hover:bg-zinc-700"
            }`}
          >
            {category.label}
            <span className="ml-1.5 opacity-60">{category.count}</span>
          </button>
        ))}
      </div>

      <ul className="flex flex-col gap-2">
        {items.map((template) => (
          <li
            key={template.id}
            className="flex items-start gap-3 rounded border border-zinc-800 bg-zinc-900 p-3"
          >
            <div className="min-w-0 flex-1">
              <p className="text-sm font-medium text-zinc-100">{template.name}</p>
              {template.description && (
                <p className="mt-0.5 text-xs text-zinc-500">{template.description}</p>
              )}
              <p className="mt-1 text-[11px] text-zinc-600">
                modo {template.mode}
                {template.source === "user" ? " · seu modelo" : ""}
              </p>
            </div>
            <div className="flex shrink-0 items-center gap-1.5">
              {template.source === "user" && (
                <button
                  type="button"
                  onClick={() => setPendingDelete(template)}
                  className="rounded px-2 py-1 text-xs text-zinc-500 hover:text-red-400"
                >
                  Excluir
                </button>
              )}
              <button
                type="button"
                disabled={transcriptionId === null || generating}
                onClick={() =>
                  transcriptionId !== null &&
                  void generateFromTemplate(transcriptionId, template.id)
                }
                className="rounded bg-zinc-800 px-2.5 py-1 text-xs text-zinc-100 hover:bg-zinc-700 disabled:opacity-40"
              >
                Usar este modelo
              </button>
            </div>
          </li>
        ))}
      </ul>

      <ConfirmDialog
        open={pendingDelete !== null}
        title="Excluir modelo"
        message={`O modelo "${pendingDelete?.name ?? ""}" será excluído definitivamente.`}
        confirmLabel="Excluir modelo"
        danger
        onConfirm={() => void handleDelete()}
        onCancel={() => setPendingDelete(null)}
      />
    </section>
  );
}
