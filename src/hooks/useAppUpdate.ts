import { useState, useEffect, useCallback } from "react";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

export type UpdateStatus =
  | "idle"
  | "checking"
  | "available"
  | "downloading"
  | "ready"
  | "error"
  | "up-to-date";

interface UpdateState {
  status: UpdateStatus;
  version: string | null;
  error: string | null;
  downloadProgress: number;
  contentLength: number;
}

export function useAppUpdate() {
  const [state, setState] = useState<UpdateState>({
    status: "idle",
    version: null,
    error: null,
    downloadProgress: 0,
    contentLength: 0,
  });

  const checkForUpdate = useCallback(async () => {
    setState((prev) => ({ ...prev, status: "checking", error: null }));
    try {
      const update = await check();
      if (update) {
        setState((prev) => ({
          ...prev,
          status: "available",
          version: update.version,
        }));
        return update;
      } else {
        setState((prev) => ({ ...prev, status: "up-to-date" }));
        return null;
      }
    } catch (error) {
      setState((prev) => ({
        ...prev,
        status: "error",
        error: String(error),
      }));
      return null;
    }
  }, []);

  const downloadAndInstall = useCallback(async () => {
    setState((prev) => ({
      ...prev,
      status: "downloading",
      downloadProgress: 0,
      contentLength: 0,
    }));
    try {
      const update = await check();
      if (!update) return;

      let downloaded = 0;
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            setState((prev) => ({
              ...prev,
              contentLength: event.data.contentLength ?? 0,
            }));
            break;
          case "Progress":
            downloaded += event.data.chunkLength;
            setState((prev) => ({
              ...prev,
              downloadProgress: downloaded,
            }));
            break;
          case "Finished":
            setState((prev) => ({ ...prev, status: "ready" }));
            break;
        }
      });

      setState((prev) => ({ ...prev, status: "ready" }));
    } catch (error) {
      setState((prev) => ({
        ...prev,
        status: "error",
        error: String(error),
      }));
    }
  }, []);

  const installAndRestart = useCallback(async () => {
    await relaunch();
  }, []);

  const dismiss = useCallback(() => {
    setState({
      status: "idle",
      version: null,
      error: null,
      downloadProgress: 0,
      contentLength: 0,
    });
  }, []);

  // Auto-check on mount (with a small delay to not block app startup)
  useEffect(() => {
    const timer = setTimeout(() => {
      checkForUpdate();
    }, 3000);
    return () => clearTimeout(timer);
  }, [checkForUpdate]);

  return {
    ...state,
    checkForUpdate,
    downloadAndInstall,
    installAndRestart,
    dismiss,
  };
}
