import { useState } from "react";
import { commands } from "../../ipc/bindings";

interface SaveAsTemplateDialogProps {
  open: boolean;
  promptId: number;
  onSaved: (name: string) => void;
  onCancel: () => void;
}

/**
 * "Salvar como modelo" (PRODUCT-SPEC §5.5): grava o prompt atual em `prompt_templates` para
 * reutilizar numa próxima gravação. O texto salvo é o que está no banco, então o editor precisa
 * ter persistido a edição antes de abrir este diálogo.
 */
export function SaveAsTemplateDialog({
  open,
  promptId,
  onSaved,
  onCancel,
}: SaveAsTemplateDialogProps) {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!open) return null;

  async function handleSave() {
    setSaving(true);
    setError(null);
    try {
      const result = await commands.savePromptAsTemplate(promptId, name, description);
      if (result.status === "ok") {
        setName("");
        setDescription("");
        onSaved(result.data.name);
      } else {
        setError(result.error);
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4">
      <div className="w-full max-w-md rounded-lg border border-zinc-800 bg-zinc-900 p-5 shadow-xl">
        <h2 className="mb-3 text-base font-semibold text-zinc-100">Salvar como modelo</h2>

        <label htmlFor="template-name" className="mb-1 block text-sm text-zinc-300">
          Nome
        </label>
        <input
          id="template-name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="Ex.: Correção de bug no backend"
          className="mb-3 w-full rounded border border-zinc-700 bg-zinc-950 px-2 py-1.5 text-sm text-zinc-100"
        />

        <label htmlFor="template-description" className="mb-1 block text-sm text-zinc-300">
          Quando usar <span className="text-zinc-500">(opcional)</span>
        </label>
        <input
          id="template-description"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="Uma frase que te lembre para que serve"
          className="mb-4 w-full rounded border border-zinc-700 bg-zinc-950 px-2 py-1.5 text-sm text-zinc-100"
        />

        {error && <p className="mb-3 text-sm text-red-400">{error}</p>}

        <div className="flex justify-end gap-2">
          <button
            type="button"
            onClick={onCancel}
            disabled={saving}
            className="rounded px-3 py-1.5 text-sm text-zinc-300 hover:bg-zinc-800 disabled:opacity-50"
          >
            Cancelar
          </button>
          <button
            type="button"
            onClick={() => void handleSave()}
            disabled={saving || name.trim() === ""}
            className="rounded bg-zinc-100 px-3 py-1.5 text-sm text-zinc-900 hover:bg-white disabled:opacity-50"
          >
            {saving ? "Salvando…" : "Salvar modelo"}
          </button>
        </div>
      </div>
    </div>
  );
}
