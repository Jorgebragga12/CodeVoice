import { create } from "zustand";
import { commands, type NewProject, type Project, type ProjectUpdate } from "../ipc/bindings";

const UNREACHABLE_ERROR = "Não foi possível falar com o backend do CodeVoice.";

interface ProjectStoreState {
  projects: Project[];
  loading: boolean;
  error: string | null;
  load: () => Promise<void>;
  create: (input: NewProject) => Promise<Project | null>;
  update: (id: number, input: ProjectUpdate) => Promise<Project | null>;
  remove: (id: number) => Promise<boolean>;
}

/**
 * Store fino (ARCHITECTURE.md §4): só orquestra chamadas IPC via `ipc/bindings` e guarda o
 * estado da lista de projetos. Nenhuma lógica de domínio mora aqui — isso vive no Rust.
 */
export const useProjectStore = create<ProjectStoreState>((set, get) => ({
  projects: [],
  loading: false,
  error: null,

  async load() {
    set({ loading: true, error: null });
    try {
      const result = await commands.listProjects();
      if (result.status === "ok") {
        set({ projects: result.data, loading: false });
      } else {
        set({ error: result.error, loading: false });
      }
    } catch {
      set({ error: UNREACHABLE_ERROR, loading: false });
    }
  },

  async create(input) {
    set({ error: null });
    try {
      const result = await commands.createProject(input);
      if (result.status === "ok") {
        await get().load();
        return result.data;
      }
      set({ error: result.error });
      return null;
    } catch {
      set({ error: UNREACHABLE_ERROR });
      return null;
    }
  },

  async update(id, input) {
    set({ error: null });
    try {
      const result = await commands.updateProject(id, input);
      if (result.status === "ok") {
        await get().load();
        return result.data;
      }
      set({ error: result.error });
      return null;
    } catch {
      set({ error: UNREACHABLE_ERROR });
      return null;
    }
  },

  async remove(id) {
    set({ error: null });
    try {
      const result = await commands.deleteProject(id);
      if (result.status === "ok") {
        await get().load();
        return true;
      }
      set({ error: result.error });
      return false;
    } catch {
      set({ error: UNREACHABLE_ERROR });
      return false;
    }
  },
}));
