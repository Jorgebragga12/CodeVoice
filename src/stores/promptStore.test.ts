import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { usePromptStore, UNDO_COALESCE_MS } from "./promptStore";
import { commands } from "../ipc/bindings";

vi.mock("../ipc/bindings", () => ({
  commands: {
    generatePrompt: vi.fn(),
    generatePromptFromTemplate: vi.fn(),
    refinePrompt: vi.fn(),
    updatePromptContent: vi.fn(),
  },
}));

const mocked = vi.mocked(commands as unknown as Record<string, ReturnType<typeof vi.fn>>);

function prompt(content: string, original = content) {
  return {
    id: 7,
    transcription_id: 42,
    project_id: null,
    mode: "technical",
    generator: "template",
    content,
    original_content: original,
    created_at: "2026-07-29T00:00:00Z",
    updated_at: "2026-07-29T00:00:00Z",
  };
}

/** Cada edição espaçada no tempo vira um nível de desfazer (fora da janela de coalescência). */
function editApart(text: string) {
  vi.advanceTimersByTime(UNDO_COALESCE_MS + 1);
  usePromptStore.getState().setContent(text);
}

describe("promptStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.useFakeTimers();
    usePromptStore.getState().reset();
    usePromptStore.setState({ prompt: prompt("v0"), content: "v0" });
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("keeps at least 10 undo levels", () => {
    for (let i = 1; i <= 12; i++) editApart(`v${i}`);

    const store = usePromptStore.getState();
    expect(store.content).toBe("v12");
    expect(store.undoStack.length).toBeGreaterThanOrEqual(10);

    // Desfazendo 12 vezes volta ao texto original, passo a passo.
    for (let i = 11; i >= 0; i--) {
      usePromptStore.getState().undo();
      expect(usePromptStore.getState().content).toBe(`v${i}`);
    }
  });

  it("does not undo past the first known state", () => {
    editApart("v1");
    usePromptStore.getState().undo();
    usePromptStore.getState().undo();

    expect(usePromptStore.getState().content).toBe("v0");
  });

  /** Sem coalescência, cada tecla viraria um nível e desfazer não devolveria nada útil. */
  it("coalesces edits typed in quick succession into one level", () => {
    editApart("frase A");
    editApart("frase B");
    // Rajada de digitação: sem pausa entre as chamadas.
    usePromptStore.getState().setContent("frase B m");
    usePromptStore.getState().setContent("frase B ma");
    usePromptStore.getState().setContent("frase B maior");

    // Dois níveis (v0 e "frase A"): a rajada inteira não acrescentou nenhum.
    expect(usePromptStore.getState().undoStack).toEqual(["v0", "frase A"]);
    usePromptStore.getState().undo();
    expect(usePromptStore.getState().content).toBe("frase A");
  });

  it("reverts to the prompt as it was generated", () => {
    usePromptStore.setState({ prompt: prompt("editado", "gerado"), content: "editado" });

    usePromptStore.getState().revertToOriginal();

    expect(usePromptStore.getState().content).toBe("gerado");
    // Reverter também é desfazível.
    usePromptStore.getState().undo();
    expect(usePromptStore.getState().content).toBe("editado");
  });

  it("persists the edited text and clears the dirty flag", async () => {
    mocked.updatePromptContent.mockResolvedValue({ status: "ok", data: prompt("editado", "v0") });
    editApart("editado");

    const saved = await usePromptStore.getState().save();

    expect(mocked.updatePromptContent).toHaveBeenCalledWith(7, "editado");
    expect(saved).toBe("editado");
    expect(usePromptStore.getState().dirty).toBe(false);
  });

  it("does not hit the backend when there is nothing to save", async () => {
    await usePromptStore.getState().save();
    expect(mocked.updatePromptContent).not.toHaveBeenCalled();
  });

  /** O backend refina o que está no banco; refinar sem salvar descartaria a edição pendente. */
  it("saves pending edits before refining", async () => {
    const calls: string[] = [];
    mocked.updatePromptContent.mockImplementation(() => {
      calls.push("save");
      return Promise.resolve({ status: "ok", data: prompt("editado", "v0") });
    });
    mocked.refinePrompt.mockImplementation(() => {
      calls.push("refine");
      return Promise.resolve({
        status: "ok",
        data: { prompt: prompt("refinado", "v0"), fallback_reason: null },
      });
    });
    editApart("editado");

    await usePromptStore.getState().refine("shorten");

    expect(calls).toEqual(["save", "refine"]);
    expect(usePromptStore.getState().content).toBe("refinado");
  });

  it("makes a refine undoable", async () => {
    mocked.refinePrompt.mockResolvedValue({
      status: "ok",
      data: { prompt: prompt("refinado", "v0"), fallback_reason: null },
    });

    await usePromptStore.getState().refine("more_technical");
    usePromptStore.getState().undo();

    expect(usePromptStore.getState().content).toBe("v0");
  });

  it("surfaces the fallback warning when refining without the claude CLI", async () => {
    mocked.refinePrompt.mockResolvedValue({
      status: "ok",
      data: { prompt: prompt("v0"), fallback_reason: "Não foi possível refinar pelo Claude." },
    });

    await usePromptStore.getState().refine("expand");

    expect(usePromptStore.getState().fallbackReason).toMatch(/Não foi possível refinar/);
    expect(usePromptStore.getState().refining).toBeNull();
  });

  it("loads a prompt generated from a saved template", async () => {
    mocked.generatePromptFromTemplate.mockResolvedValue({
      status: "ok",
      data: { prompt: prompt("do modelo"), fallback_reason: null },
    });

    await usePromptStore.getState().generateFromTemplate(42, 3);

    expect(mocked.generatePromptFromTemplate).toHaveBeenCalledWith(42, 3);
    expect(usePromptStore.getState().content).toBe("do modelo");
    expect(usePromptStore.getState().dirty).toBe(false);
  });
});
