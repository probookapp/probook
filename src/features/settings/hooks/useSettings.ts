import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { settingsApi, backupApi } from "@/lib/tauri";
import type { UpdateCompanySettingsInput } from "@/types";

export function useCompanySettings() {
  return useQuery({
    queryKey: ["company-settings"],
    queryFn: settingsApi.get,
  });
}

export function useUpdateCompanySettings() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: UpdateCompanySettingsInput) => settingsApi.update(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["company-settings"] });
    },
  });
}

export function useUpdateAppSettings() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ appLanguage, appTheme, autoUpdateEnabled }: { appLanguage: string; appTheme: string; autoUpdateEnabled: boolean }) =>
      settingsApi.updateAppSettings(appLanguage, appTheme, autoUpdateEnabled),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["company-settings"] });
    },
  });
}

export function useUploadLogo() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (filePath: string) => settingsApi.uploadLogo(filePath),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["company-settings"] });
      queryClient.invalidateQueries({ queryKey: ["logo-base64"] });
    },
  });
}

export function useLogoBase64() {
  return useQuery({
    queryKey: ["logo-base64"],
    queryFn: settingsApi.getLogoBase64,
  });
}

export function useDeleteLogo() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () => settingsApi.deleteLogo(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["company-settings"] });
      queryClient.invalidateQueries({ queryKey: ["logo-base64"] });
    },
  });
}

export function useExportBackup() {
  return useMutation({
    mutationFn: ({ filePath, password }: { filePath: string; password: string }) =>
      backupApi.export(filePath, password),
  });
}

export function useImportBackup() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ filePath, password }: { filePath: string; password: string }) =>
      backupApi.import(filePath, password),
    onSuccess: () => {
      queryClient.invalidateQueries();
    },
  });
}

export function useCreateLocalBackup() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () => backupApi.createLocalBackup(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["company-settings"] });
      queryClient.invalidateQueries({ queryKey: ["backup-list"] });
    },
  });
}

export function useBackupList() {
  return useQuery({
    queryKey: ["backup-list"],
    queryFn: backupApi.getBackupList,
  });
}

export function useOpenBackupsFolder() {
  return useMutation({
    mutationFn: () => backupApi.openBackupsFolder(),
  });
}

export function useDeleteBackup() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (path: string) => backupApi.deleteBackup(path),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["backup-list"] });
    },
  });
}
