import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { useProjectStore } from "../../stores/projectStore";
import { ProjectForm } from "./ProjectForm";

const { commands } = vi.hoisted(() => ({
  commands: {
    listProjects: vi.fn(),
    getProject: vi.fn(),
    createProject: vi.fn(),
    updateProject: vi.fn(),
    deleteProject: vi.fn(),
    listProjectRules: vi.fn(),
    createProjectRule: vi.fn(),
    updateProjectRule: vi.fn(),
    deleteProjectRule: vi.fn(),
    reorderProjectRules: vi.fn(),
    validateProjectPath: vi.fn(),
    previewProjectImport: vi.fn(),
  },
}));

vi.mock("../../ipc/bindings", () => ({ commands }));

describe("ProjectForm", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useProjectStore.setState({ projects: [], loading: false, error: null });
  });

  it("blocks submit with a client-side error when name/path are blank", async () => {
    const user = userEvent.setup();
    const onSaved = vi.fn();

    render(<ProjectForm onSaved={onSaved} onCancel={vi.fn()} />);

    // Espaços em branco passam pelo `required` nativo do input, mas não pelo `.trim()` do form.
    await user.type(screen.getByLabelText("Nome"), " ");
    await user.type(screen.getByPlaceholderText("C:\\projects\\meu-projeto"), " ");
    await user.click(screen.getByRole("button", { name: "Salvar projeto" }));

    expect(await screen.findByText("Nome e caminho são obrigatórios.")).toBeInTheDocument();
    expect(commands.createProject).not.toHaveBeenCalled();
    expect(onSaved).not.toHaveBeenCalled();
  });

  it("creates a project with the entered fields", async () => {
    const user = userEvent.setup();
    const onSaved = vi.fn();
    commands.createProject.mockResolvedValue({
      status: "ok",
      data: {
        id: 1,
        name: "CodeVoice",
        path: "C:\\projects\\codevoice",
        description: "",
        stack: "",
        architecture: "",
        dev_commands: "",
        test_commands: "",
        forbidden_tech: "",
        database_info: "",
        notes: "",
        created_at: "2026-07-22T00:00:00Z",
        updated_at: "2026-07-22T00:00:00Z",
      },
    });
    commands.listProjects.mockResolvedValue({ status: "ok", data: [] });

    render(<ProjectForm onSaved={onSaved} onCancel={vi.fn()} />);

    await user.type(screen.getByLabelText("Nome"), "CodeVoice");
    await user.type(
      screen.getByPlaceholderText("C:\\projects\\meu-projeto"),
      "C:\\projects\\codevoice",
    );
    await user.click(screen.getByRole("button", { name: "Salvar projeto" }));

    await waitFor(() =>
      expect(commands.createProject).toHaveBeenCalledWith(
        expect.objectContaining({ name: "CodeVoice", path: "C:\\projects\\codevoice" }),
      ),
    );
    await waitFor(() => expect(onSaved).toHaveBeenCalled());
  });

  it("previews an import and lets the user apply the content to notes", async () => {
    const user = userEvent.setup();
    commands.previewProjectImport.mockResolvedValue({
      status: "ok",
      data: {
        root: "C:\\projects\\codevoice",
        files: [{ relative_path: "README.md", size_bytes: 12, content: "# CodeVoice" }],
        directories: ["src"],
      },
    });

    render(<ProjectForm onSaved={vi.fn()} onCancel={vi.fn()} />);

    await user.type(
      screen.getByPlaceholderText("C:\\projects\\meu-projeto"),
      "C:\\projects\\codevoice",
    );
    await user.click(screen.getByRole("button", { name: /pré-visualizar importação/i }));

    expect(await screen.findByText(/1 arquivo\(s\) lido\(s\)/i)).toBeInTheDocument();
    expect(screen.getByText("README.md", { exact: false })).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: /usar conteúdo nas notas/i }));

    const notes = screen.getByLabelText("Notas") as HTMLTextAreaElement;
    expect(notes.value).toContain("README.md");
    expect(notes.value).toContain("# CodeVoice");
  });

  it("shows the backend validation error when the path is invalid", async () => {
    const user = userEvent.setup();
    commands.validateProjectPath.mockResolvedValue({
      status: "error",
      error: "caminho contém um segmento de travessia de diretório",
    });

    render(<ProjectForm onSaved={vi.fn()} onCancel={vi.fn()} />);

    const pathInput = screen.getByPlaceholderText("C:\\projects\\meu-projeto");
    await user.type(pathInput, "C:\\projects\\..\\Windows");
    await user.tab();

    expect(await screen.findByText(/travessia de diretório/i)).toBeInTheDocument();
  });
});
