interface ProjectContextFieldsValue {
  stack: string;
  architecture: string;
  dev_commands: string;
  test_commands: string;
  forbidden_tech: string;
  database_info: string;
}

interface ProjectContextFieldsProps {
  value: ProjectContextFieldsValue;
  onChange: (key: keyof ProjectContextFieldsValue, value: string) => void;
}

const inputClass = "rounded border border-zinc-700 bg-zinc-950 px-2 py-1 text-zinc-100";
const monoTextareaClass = `${inputClass} font-mono text-xs`;

/**
 * Campos de contexto do projeto (stack, arquitetura, comandos, etc.) — extraídos de
 * `ProjectForm` só pra manter os dois arquivos dentro do limite de ~300 linhas.
 */
export function ProjectContextFields({ value, onChange }: ProjectContextFieldsProps) {
  return (
    <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
      <label className="flex flex-col gap-1">
        <span className="text-zinc-400">Stack</span>
        <input
          value={value.stack}
          onChange={(e) => onChange("stack", e.target.value)}
          className={inputClass}
        />
      </label>
      <label className="flex flex-col gap-1">
        <span className="text-zinc-400">Arquitetura</span>
        <input
          value={value.architecture}
          onChange={(e) => onChange("architecture", e.target.value)}
          className={inputClass}
        />
      </label>
      <label className="flex flex-col gap-1">
        <span className="text-zinc-400">Comandos de dev (1 por linha)</span>
        <textarea
          value={value.dev_commands}
          onChange={(e) => onChange("dev_commands", e.target.value)}
          rows={2}
          className={monoTextareaClass}
        />
      </label>
      <label className="flex flex-col gap-1">
        <span className="text-zinc-400">Comandos de teste (1 por linha)</span>
        <textarea
          value={value.test_commands}
          onChange={(e) => onChange("test_commands", e.target.value)}
          rows={2}
          className={monoTextareaClass}
        />
      </label>
      <label className="flex flex-col gap-1">
        <span className="text-zinc-400">Tecnologias proibidas</span>
        <input
          value={value.forbidden_tech}
          onChange={(e) => onChange("forbidden_tech", e.target.value)}
          className={inputClass}
        />
      </label>
      <label className="flex flex-col gap-1">
        <span className="text-zinc-400">Banco de dados</span>
        <input
          value={value.database_info}
          onChange={(e) => onChange("database_info", e.target.value)}
          className={inputClass}
        />
      </label>
    </div>
  );
}
