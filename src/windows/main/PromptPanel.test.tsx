import { beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";

import { PromptPanel } from "./PromptPanel";
import { usePromptStore } from "../../stores/promptStore";
import { useTranscriptionStore } from "../../stores/transcriptionStore";
import { commands } from "../../ipc/bindings";

vi.mock("../../ipc/bindings", () => ({
  commands: {
    listPromptModes: vi.fn(),
    claudeCliAvailable: vi.fn(),
    generatePrompt: vi.fn(),
  },
}));

const mocked = vi.mocked(commands as unknown as Record<string, ReturnType<typeof vi.fn>>);

/** Coloca uma transcrição pronta no store — é a pré-condição para o painel aparecer. */
function withReadyTranscription(id = 42) {
  useTranscriptionStore.setState({ phase: "done", transcriptionId: id, text: "criar login" });
}

function promptResult(overrides: Record<string, unknown> = {}) {
  return {
    status: "ok" as const,
    data: {
      prompt: {
        id: 1,
        transcription_id: 42,
        project_id: null,
        mode: "technical",
        generator: "template",
        content: "# Prompt técnico\n\n## Objetivo\ncriar login",
        original_content: "# Prompt técnico\n\n## Objetivo\ncriar login",
        created_at: "2026-07-24T00:00:00Z",
        updated_at: "2026-07-24T00:00:00Z",
      },
      fallback_reason: null,
      ...overrides,
    },
  };
}

describe("PromptPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    usePromptStore.setState({
      generating: false,
      prompt: null,
      fallbackReason: null,
      error: null,
      mode: "technical",
    });
    useTranscriptionStore.setState({ phase: "idle", transcriptionId: null, text: null });

    mocked.listPromptModes.mockResolvedValue({
      status: "ok",
      data: [
        { id: "technical", label: "Prompt técnico", description: "Estrutura completa." },
        { id: "bug_fix", label: "Correção de erro", description: "Investigar e corrigir." },
      ],
    });
    mocked.claudeCliAvailable.mockResolvedValue({ status: "ok", data: true });
  });

  it("stays hidden until a transcription is ready", () => {
    render(<PromptPanel />);
    expect(screen.queryByText("Gerar prompt")).not.toBeInTheDocument();
  });

  it("shows the mode selector once a transcription exists", async () => {
    withReadyTranscription();
    render(<PromptPanel />);

    expect(await screen.findByText("Gerar prompt")).toBeInTheDocument();
    expect(await screen.findByRole("option", { name: "Prompt técnico" })).toBeInTheDocument();
  });

  it("generates and displays the prompt", async () => {
    mocked.generatePrompt.mockResolvedValue(promptResult());
    withReadyTranscription();
    render(<PromptPanel />);

    await userEvent.click(await screen.findByText("Gerar prompt"));

    await waitFor(() => {
      expect(mocked.generatePrompt).toHaveBeenCalledWith(42, "technical");
    });
    // O textarea do resultado (o <select> de modos também exibiria "Prompt técnico").
    const output = await screen.findByRole("textbox");
    expect(output).toHaveValue("# Prompt técnico\n\n## Objetivo\ncriar login");
  });

  it("warns the user when the prompt came from the template fallback", async () => {
    mocked.generatePrompt.mockResolvedValue(
      promptResult({ fallback_reason: "Gerado por modelo local de template (CLI indisponível)." }),
    );
    withReadyTranscription();
    render(<PromptPanel />);

    await userEvent.click(await screen.findByText("Gerar prompt"));

    expect(await screen.findByText(/modelo local de template/)).toBeInTheDocument();
  });

  it("warns up front when the claude CLI is missing", async () => {
    mocked.claudeCliAvailable.mockResolvedValue({ status: "ok", data: false });
    withReadyTranscription();
    render(<PromptPanel />);

    expect(await screen.findByText(/não foi encontrado/)).toBeInTheDocument();
  });

  it("surfaces generation errors instead of failing silently", async () => {
    mocked.generatePrompt.mockResolvedValue({ status: "error", error: "transcrição vazia" });
    withReadyTranscription();
    render(<PromptPanel />);

    await userEvent.click(await screen.findByText("Gerar prompt"));

    expect(await screen.findByText("transcrição vazia")).toBeInTheDocument();
  });
});
