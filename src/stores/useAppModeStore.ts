import { create } from "zustand";
import { persist } from "zustand/middleware";

export type AppMode = "office" | "pos";

interface AppModeState {
  mode: AppMode;
  setMode: (mode: AppMode) => void;
  toggleMode: () => void;
}

export const useAppModeStore = create<AppModeState>()(
  persist(
    (set, get) => ({
      mode: "office",
      setMode: (mode) => set({ mode }),
      toggleMode: () =>
        set({ mode: get().mode === "office" ? "pos" : "office" }),
    }),
    {
      name: "probook-app-mode",
    }
  )
);
