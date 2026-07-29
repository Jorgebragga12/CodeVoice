import { beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";

import { TemplateLibrary } from "./TemplateLibrary";
import { usePromptStore } from "../../stores/promptStore";
import { useTranscriptionStore } from "../../stores/transcriptionStore";
import { commands } from "../../ipc/bindings";

vi.mock("../../ipc/bindings", () => ({
  commands: {
    listTemplateCategories: vi.fn(),
    listPromptTemplates: vi.fn(),
    deletePromptTemplate: vi.fn(),
    generatePromptFromTemplate: vi.fn(),
  },
}));

const mocked = vi.mocked(commands as unknown as Record<string, ReturnType<typeof vi.fn>>);

function template(overrides: Record<string, unknown> = {}) {
  return {
    id: 1,
    name: "Erro agora",
    mode: "bug_fix",
    category: "depuracao",
    description: "quebrou agora, tenho a mensagem de erro na tela",
    content: "corpo <<SUA FALA>>",
    source: "builtin",
    slug: "depuracao/erro-agora",
    project_id: null,
    created_at: "2026-07-29T00:00:00Z",
    updated_at: "2026-07-29T00:00:00Z",
    ...overrides,
  };
}

describe("TemplateLibrary", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    usePromptStore.getState().reset();
    useTranscriptionStore.setState({ phase: "done", transcriptionId: 42, text: "quebrou o login" });

    mocked.listTemplateCategories.mockResolvedValue({
      status: "ok",
      data: [
        { id: "meus-modelos", label: "Meus modelos", count: 1 },
        { id: "depuracao", label: "Depuração", count: 8 },
      ],
    });
    mocked.listPromptTemplates.mockResolvedValue({ status: "ok", data: [template()] });
  });

  it("lists the categories with their counts", async () => {
    render(<TemplateLibrary />);

    expect(await screen.findByRole("button", { name: /Depuração/ })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Meus modelos/ })).toHaveTextContent("1");
  });

  it("shows the template name and what it is for", async () => {
    render(<TemplateLibrary />);

    expect(await screen.findByText("Erro agora")).toBeInTheDocument();
    expect(screen.getByText(/quebrou agora/)).toBeInTheDocument();
  });

  it("generates a prompt from the chosen template", async () => {
    mocked.generatePromptFromTemplate.mockResolvedValue({
      status: "ok",
      data: { prompt: { id: 9, content: "pronto" }, fallback_reason: null },
    });
    const user = userEvent.setup();
    render(<TemplateLibrary />);

    await user.click(await screen.findByRole("button", { name: "Usar este modelo" }));

    await waitFor(() => {
      expect(mocked.generatePromptFromTemplate).toHaveBeenCalledWith(42, 1);
    });
  });

  it("cannot use a template before there is a transcription", async () => {
    useTranscriptionStore.setState({ phase: "idle", transcriptionId: null, text: null });
    render(<TemplateLibrary />);

    expect(await screen.findByRole("button", { name: "Usar este modelo" })).toBeDisabled();
  });

  /** Modelos embutidos voltariam no próximo startup — oferecer excluir seria enganoso. */
  it("offers deletion only for the user's own templates", async () => {
    render(<TemplateLibrary />);
    await screen.findByText("Erro agora");
    expect(screen.queryByRole("button", { name: "Excluir" })).not.toBeInTheDocument();
  });

  it("asks for confirmation before deleting a user template", async () => {
    mocked.listPromptTemplates.mockResolvedValue({
      status: "ok",
      data: [template({ id: 5, name: "Meu fluxo", source: "user", slug: null })],
    });
    mocked.deletePromptTemplate.mockResolvedValue({ status: "ok", data: null });
    const user = userEvent.setup();
    render(<TemplateLibrary />);

    await user.click(await screen.findByRole("button", { name: "Excluir" }));
    expect(mocked.deletePromptTemplate).not.toHaveBeenCalled();

    await user.click(await screen.findByRole("button", { name: "Excluir modelo" }));

    await waitFor(() => expect(mocked.deletePromptTemplate).toHaveBeenCalledWith(5));
  });
});
