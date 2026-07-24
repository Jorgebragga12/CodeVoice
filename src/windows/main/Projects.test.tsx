import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { Project } from "../../ipc/bindings";
import { useProjectStore } from "../../stores/projectStore";
import { Projects } from "./Projects";

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

function sampleProject(overrides: Partial<Project> = {}): Project {
  return {
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
    ...overrides,
  };
}

describe("Projects", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useProjectStore.setState({ projects: [], loading: false, error: null });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("shows an empty state when there are no projects", async () => {
    commands.listProjects.mockResolvedValue({ status: "ok", data: [] });

    render(<Projects />);

    expect(await screen.findByText(/nenhum projeto cadastrado/i)).toBeInTheDocument();
  });

  it("renders the project list once loaded", async () => {
    commands.listProjects.mockResolvedValue({ status: "ok", data: [sampleProject()] });

    render(<Projects />);

    expect(await screen.findByText("CodeVoice")).toBeInTheDocument();
    expect(screen.getByText("C:\\projects\\codevoice")).toBeInTheDocument();
  });

  it("asks for confirmation before deleting and only deletes after confirming", async () => {
    const user = userEvent.setup();
    commands.listProjects.mockResolvedValue({ status: "ok", data: [sampleProject()] });
    commands.deleteProject.mockResolvedValue({ status: "ok", data: null });

    render(<Projects />);
    await screen.findByText("CodeVoice");

    await user.click(screen.getByRole("button", { name: "Excluir" }));

    const dialogHeading = await screen.findByRole("heading", { name: "Excluir projeto" });
    expect(dialogHeading).toBeInTheDocument();
    expect(commands.deleteProject).not.toHaveBeenCalled();

    const confirmButton = screen.getByRole("button", { name: "Excluir projeto" });
    await user.click(confirmButton);

    await waitFor(() => expect(commands.deleteProject).toHaveBeenCalledWith(1));
  });

  it("cancelling the delete confirmation does not call the backend", async () => {
    const user = userEvent.setup();
    commands.listProjects.mockResolvedValue({ status: "ok", data: [sampleProject()] });

    render(<Projects />);
    await screen.findByText("CodeVoice");

    await user.click(screen.getByRole("button", { name: "Excluir" }));
    await screen.findByRole("heading", { name: "Excluir projeto" });

    await user.click(screen.getByRole("button", { name: "Cancelar" }));

    await waitFor(() =>
      expect(screen.queryByRole("heading", { name: "Excluir projeto" })).not.toBeInTheDocument(),
    );
    expect(commands.deleteProject).not.toHaveBeenCalled();
  });

  it("opens the create form when clicking the new project button", async () => {
    const user = userEvent.setup();
    commands.listProjects.mockResolvedValue({ status: "ok", data: [] });

    render(<Projects />);
    await screen.findByText(/nenhum projeto cadastrado/i);

    await user.click(screen.getByRole("button", { name: "+ Novo projeto" }));

    expect(screen.getByRole("heading", { name: "Novo projeto" })).toBeInTheDocument();
  });

  it("opens the edit form pre-filled when clicking edit on a project", async () => {
    const user = userEvent.setup();
    commands.listProjects.mockResolvedValue({ status: "ok", data: [sampleProject()] });
    commands.listProjectRules.mockResolvedValue({ status: "ok", data: [] });

    render(<Projects />);
    await screen.findByText("CodeVoice");

    await user.click(screen.getByRole("button", { name: "Editar" }));

    const heading = screen.getByRole("heading", { name: /editar codevoice/i });
    expect(heading).toBeInTheDocument();

    const dialog = heading.closest("div");
    expect(dialog).not.toBeNull();
    if (dialog) {
      expect(within(dialog).getByDisplayValue("C:\\projects\\codevoice")).toBeInTheDocument();
    }
  });
});
