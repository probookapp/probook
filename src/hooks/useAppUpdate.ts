import { useState, useEffect, useCallback } from "react";
import { isTauri } from "@/lib/config";
import { useCompanySettings } from "@/features/settings/hooks/useSettings";

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
  autoMode: boolean;
}

export function useAppUpdate() {
  const { data: settings } = useCompanySettings();
  const autoUpdateEnabled = settings?.auto_update_enabled ?? true;

  const [state, setState] = useState<UpdateState>({
    status: "idle",
    version: null,
    error: null,
    downloadProgress: 0,
    contentLength: 0,
    autoMode: false,
  });

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const downloadAndInstallUpdate = useCallback(async (updateObj: any) => {
    if (!updateObj || !isTauri()) return;

    setState((prev) => ({
      ...prev,
      status: "downloading",
      downloadProgress: 0,
      contentLength: 0,
    }));

    let downloaded = 0;
    await updateObj.downloadAndInstall((event: { event: string; data: { contentLength?: number; chunkLength: number } }) => {
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
  }, []);

  const checkForUpdate = useCallback(async () => {
    if (!isTauri()) {
      setState((prev) => ({ ...prev, status: "up-to-date" }));
      return null;
    }

    setState((prev) => ({ ...prev, status: "checking", error: null }));
    try {
      const { check } = await import("@tauri-apps/plugin-updater");
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
    if (!isTauri()) return;
    try {
      const { check } = await import("@tauri-apps/plugin-updater");
      const update = await check();
      if (!update) return;
      await downloadAndInstallUpdate(update);
    } catch (error) {
      setState((prev) => ({
        ...prev,
        status: "error",
        error: String(error),
      }));
    }
  }, [downloadAndInstallUpdate]);

  const installAndRestart = useCallback(async () => {
    if (!isTauri()) return;
    const { relaunch } = await import("@tauri-apps/plugin-process");
    await relaunch();
  }, []);

  const dismiss = useCallback(() => {
    setState({
      status: "idle",
      version: null,
      error: null,
      downloadProgress: 0,
      contentLength: 0,
      autoMode: false,
    });
  }, []);

  // Auto-check on mount (with a small delay to not block app startup)
  useEffect(() => {
    if (!isTauri()) return;

    const timer = setTimeout(async () => {
      const update = await checkForUpdate();

      // If auto-update is enabled and an update is found, auto-download and install
      if (update && autoUpdateEnabled) {
        setState((prev) => ({ ...prev, autoMode: true }));
        try {
          await downloadAndInstallUpdate(update);
          // Auto-relaunch after install
          const { relaunch } = await import("@tauri-apps/plugin-process");
          await relaunch();
        } catch (error) {
          setState((prev) => ({
            ...prev,
            status: "error",
            error: String(error),
            autoMode: false,
          }));
        }
      }
    }, 3000);
    return () => clearTimeout(timer);
  }, [checkForUpdate, autoUpdateEnabled, downloadAndInstallUpdate]);

  return {
    ...state,
    autoUpdateEnabled,
    checkForUpdate,
    downloadAndInstall,
    installAndRestart,
    dismiss,
  };
}
