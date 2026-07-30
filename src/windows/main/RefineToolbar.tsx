import type { RefineAction } from "../../ipc/bindings";

const REFINE_ACTIONS: { action: RefineAction; label: string; title: string }[] = [
  { action: "shorten", label: "Encurtar", title: "Corta redundância mantendo os requisitos" },
  {
    action: "expand",
    label: "Detalhar",
    title: "Explicita casos de borda e critérios de aceitação",
  },
  {
    action: "more_technical",
    label: "Mais técnico",
    title: "Reescreve com os termos corretos da stack, sem mudar o escopo",
  },
  {
    action: "split_into_steps",
    label: "Dividir em etapas",
    title: "Quebra em passos numerados e verificáveis isoladamente",
  },
];

/** Explica por que os botões estão apagados, no lugar da descrição da ação. */
const UNAVAILABLE_TITLE =
  "Indisponível: refinar exige o comando `claude`, que não foi encontrado no PATH";

interface RefineToolbarProps {
  refining: RefineAction | null;
  busy: boolean;
  canUndo: boolean;
  canRevert: boolean;
  /** `false` desabilita as 4 ações de refino — sem o `claude` CLI elas não têm efeito. */
  refineAvailable: boolean;
  onRefine: (action: RefineAction) => void;
  onUndo: () => void;
  onRevert: () => void;
}

/**
 * Ações de refino do PRODUCT-SPEC §5.5. As quatro primeiras vão ao `refine_prompt` (que usa o
 * Claude CLI); desfazer e reverter são locais e continuam funcionando sem o CLI.
 */
export function RefineToolbar({
  refining,
  busy,
  canUndo,
  canRevert,
  refineAvailable,
  onRefine,
  onUndo,
  onRevert,
}: RefineToolbarProps) {
  return (
    <div className="flex flex-wrap items-center gap-1.5">
      {REFINE_ACTIONS.map(({ action, label, title }) => (
        <button
          key={action}
          type="button"
          title={refineAvailable ? title : UNAVAILABLE_TITLE}
          disabled={busy || !refineAvailable}
          onClick={() => onRefine(action)}
          className="rounded bg-zinc-800 px-2.5 py-1 text-xs text-zinc-100 hover:bg-zinc-700 disabled:opacity-50"
        >
          {refining === action ? `${label}…` : label}
        </button>
      ))}

      <span className="mx-1 h-4 w-px bg-zinc-800" aria-hidden="true" />

      <button
        type="button"
        title="Volta ao estado anterior do texto"
        disabled={busy || !canUndo}
        onClick={onUndo}
        className="rounded bg-zinc-800 px-2.5 py-1 text-xs text-zinc-100 hover:bg-zinc-700 disabled:opacity-50"
      >
        Desfazer
      </button>
      <button
        type="button"
        title="Descarta todas as edições e volta ao prompt como foi gerado"
        disabled={busy || !canRevert}
        onClick={onRevert}
        className="rounded px-2.5 py-1 text-xs text-zinc-400 hover:text-zinc-200 disabled:opacity-50"
      >
        Voltar ao original
      </button>
    </div>
  );
}
