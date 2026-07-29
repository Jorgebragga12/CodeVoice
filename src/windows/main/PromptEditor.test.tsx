import { beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";

import { PromptEditor } from "./PromptEditor";
import { usePromptStore } from "../../stores/promptStore";
import { useTranscriptionStore } from "../../stores/transcriptionStore";
import { commands } from "../../ipc/bindings";

vi.mock("../../ipc/bindings", () => ({
  commands: {
    updatePromptContent: vi.fn(),
    refinePrompt: vi.fn(),
    savePromptAsTemplate: vi.fn(),
  },
}));

const mocked = vi.mocked(commands as unknown as Record<string, ReturnType<typeof vi.fn>>);

const GENERATED = "# Prompt técnico\n\n## Objetivo\ncriar login";

function prompt(content = GENERATED) {
  return {
    id: 7,
    transcription_id: 42,
    project_id: null,
    mode: "technical",
    generator: "template",
    content,
    original_content: GENERATED,
    created_at: "2026-07-29T00:00:00Z",
    updated_at: "2026-07-29T00:00:00Z",
  };
}

describe("PromptEditor", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    usePromptStore.getState().reset();
    usePromptStore.setState({ prompt: prompt(), content: GENERATED });
    useTranscriptionStore.setState({ phase: "done", transcriptionId: 42, text: "criar login" });

    mocked.updatePromptContent.mockImplementation((_id: number, content: string) =>
      Promise.resolve({ status: "ok", data: prompt(content) }),
    );
  });

  it("shows the original transcription read-only beside the editable prompt", () => {
    render(<PromptEditor />);

    const transcript = screen.getByLabelText("Transcrição original");
    expect(transcript).toHaveValue("criar login");
    expect(transcript).toHaveAttribute("readonly");
    expect(screen.getByLabelText("Prompt (editável)")).not.toHaveAttribute("readonly");
  });

  /** Critério de aceite da Fase 7: o clipboard leva o texto editado, e ele fica persistido. */
  it("copies the edited text and persists it", async () => {
    // `userEvent.setup()` instala o stub de clipboard do jsdom — lemos dele de volta em vez de
    // espionar a chamada, para checar o conteúdo que de fato foi para a área de transferência.
    const user = userEvent.setup();
    render(<PromptEditor />);

    const editor = screen.getByLabelText("Prompt (editável)");
    await user.clear(editor);
    await user.type(editor, "prompt editado a mão");
    await user.click(screen.getByRole("button", { name: "Copiar" }));

    await waitFor(async () => {
      await expect(navigator.clipboard.readText()).resolves.toBe("prompt editado a mão");
    });
    expect(mocked.updatePromptContent).toHaveBeenCalledWith(7, "prompt editado a mão");
  });

  it("undoes an edit back to the generated text", async () => {
    const user = userEvent.setup();
    render(<PromptEditor />);

    await user.clear(screen.getByLabelText("Prompt (editável)"));
    await user.type(screen.getByLabelText("Prompt (editável)"), "outra coisa");
    await user.click(screen.getByRole("button", { name: "Voltar ao original" }));

    expect(screen.getByLabelText("Prompt (editável)")).toHaveValue(GENERATED);
  });

  it("keeps undo disabled while nothing was changed", () => {
    render(<PromptEditor />);
    expect(screen.getByRole("button", { name: "Desfazer" })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Voltar ao original" })).toBeDisabled();
  });

  it("runs each refine action through the backend", async () => {
    mocked.refinePrompt.mockResolvedValue({
      status: "ok",
      data: { prompt: prompt("encurtado"), fallback_reason: null },
    });
    const user = userEvent.setup();
    render(<PromptEditor />);

    await user.click(screen.getByRole("button", { name: "Encurtar" }));

    await waitFor(() => expect(mocked.refinePrompt).toHaveBeenCalledWith(7, "shorten"));
    expect(screen.getByLabelText("Prompt (editável)")).toHaveValue("encurtado");
  });

  it("offers the four refine actions of the spec", () => {
    render(<PromptEditor />);
    for (const label of ["Encurtar", "Detalhar", "Mais técnico", "Dividir em etapas"]) {
      expect(screen.getByRole("button", { name: label })).toBeInTheDocument();
    }
  });

  /** O modelo é salvo a partir do banco: a edição pendente precisa ir antes do diálogo. */
  it("saves pending edits before opening the save-as-template dialog", async () => {
    const user = userEvent.setup();
    render(<PromptEditor />);

    await user.type(screen.getByLabelText("Prompt (editável)"), " extra");
    await user.click(screen.getByRole("button", { name: "Salvar como modelo" }));

    await waitFor(() => {
      expect(mocked.updatePromptContent).toHaveBeenCalledWith(7, `${GENERATED} extra`);
    });
    expect(await screen.findByRole("button", { name: "Salvar modelo" })).toBeInTheDocument();
  });

  it("stores the prompt as a reusable template", async () => {
    mocked.savePromptAsTemplate.mockResolvedValue({
      status: "ok",
      data: { id: 1, name: "Meu fluxo", source: "user" },
    });
    const user = userEvent.setup();
    render(<PromptEditor />);

    await user.click(screen.getByRole("button", { name: "Salvar como modelo" }));
    await user.type(await screen.findByLabelText("Nome"), "Meu fluxo");
    await user.click(screen.getByRole("button", { name: "Salvar modelo" }));

    await waitFor(() => {
      expect(mocked.savePromptAsTemplate).toHaveBeenCalledWith(7, "Meu fluxo", "");
    });
    expect(await screen.findByText(/Meu fluxo.*salvo/)).toBeInTheDocument();
  });
});
