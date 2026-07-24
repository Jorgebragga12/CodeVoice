import { useState } from "react";
import type { ImportPreview } from "../../ipc/bindings";

interface ImportPreviewPanelProps {
  preview: ImportPreview;
  onUseContent: (digest: string) => void;
}

function buildDigest(preview: ImportPreview): string {
  const parts = preview.files.map((file) => `--- ${file.relative_path} ---\n${file.content}`);
  if (preview.directories.length > 0) {
    parts.push(`--- diretórios encontrados ---\n${preview.directories.join("\n")}`);
  }
  return parts.join("\n\n");
}

/**
 * Mostra o que a importação assistida leu (SECURITY-MODEL.md §3) *antes* de qualquer coisa
 * ser usada — nada aqui é automático, o usuário decide se quer aproveitar o conteúdo.
 */
export function ImportPreviewPanel({ preview, onUseContent }: ImportPreviewPanelProps) {
  const [expanded, setExpanded] = useState<string | null>(null);

  return (
    <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3 text-sm">
      <div className="mb-2 flex flex-wrap items-center justify-between gap-2">
        <p className="text-zinc-300">
          {preview.files.length} arquivo(s) lido(s) · {preview.directories.length} diretório(s)
          listado(s)
        </p>
        <button
          type="button"
          onClick={() => onUseContent(buildDigest(preview))}
          disabled={preview.files.length === 0 && preview.directories.length === 0}
          className="rounded bg-zinc-800 px-2 py-1 text-xs text-zinc-200 hover:bg-zinc-700 disabled:opacity-40"
        >
          Usar conteúdo nas notas
        </button>
      </div>

      {preview.files.length === 0 ? (
        <p className="text-zinc-500">Nenhum arquivo da allowlist encontrado neste diretório.</p>
      ) : (
        <ul className="mb-2 space-y-1">
          {preview.files.map((file) => (
            <li key={file.relative_path}>
              <button
                type="button"
                onClick={() =>
                  setExpanded(expanded === file.relative_path ? null : file.relative_path)
                }
                className="text-left text-zinc-300 hover:text-zinc-100"
              >
                {expanded === file.relative_path ? "▾" : "▸"} {file.relative_path}{" "}
                <span className="text-zinc-600">({file.size_bytes} bytes)</span>
              </button>
              {expanded === file.relative_path && (
                <pre className="mt-1 max-h-40 overflow-auto rounded border border-zinc-800 bg-zinc-900 p-2 text-xs whitespace-pre-wrap text-zinc-400">
                  {file.content}
                </pre>
              )}
            </li>
          ))}
        </ul>
      )}

      {preview.directories.length > 0 && (
        <details className="text-zinc-500">
          <summary className="cursor-pointer text-zinc-400">Diretórios encontrados</summary>
          <ul className="mt-1 space-y-0.5 pl-3">
            {preview.directories.map((dir) => (
              <li key={dir}>{dir}</li>
            ))}
          </ul>
        </details>
      )}
    </div>
  );
}
