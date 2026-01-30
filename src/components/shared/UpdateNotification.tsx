import { useTranslation } from "react-i18next";
import { Download, RefreshCw, X, CheckCircle2, AlertCircle, Loader2 } from "lucide-react";
import { Button } from "@/components/ui";
import { useAppUpdate } from "@/hooks/useAppUpdate";

export function UpdateNotification() {
  const { t } = useTranslation("common");
  const {
    status,
    version,
    error,
    downloadProgress,
    contentLength,
    autoMode,
    checkForUpdate,
    downloadAndInstall,
    installAndRestart,
    dismiss,
  } = useAppUpdate();

  // Don't show anything when idle, checking, or up-to-date
  if (status === "idle" || status === "checking" || status === "up-to-date") {
    return null;
  }

  const progressPercent =
    contentLength > 0 ? Math.round((downloadProgress / contentLength) * 100) : 0;

  return (
    <div className="fixed bottom-6 right-6 z-50 max-w-sm w-full animate-in slide-in-from-bottom-4">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700 p-4">
        <div className="flex items-start gap-3">
          {/* Icon */}
          <div className="flex-shrink-0 mt-0.5">
            {status === "error" ? (
              <AlertCircle className="h-5 w-5 text-red-500" />
            ) : status === "ready" ? (
              <CheckCircle2 className="h-5 w-5 text-green-500" />
            ) : status === "downloading" && autoMode ? (
              <Loader2 className="h-5 w-5 text-primary-600 dark:text-primary-400 animate-spin" />
            ) : (
              <Download className="h-5 w-5 text-primary-600 dark:text-primary-400" />
            )}
          </div>

          {/* Content */}
          <div className="flex-1 min-w-0">
            <h4 className="text-sm font-medium text-gray-900 dark:text-gray-100">
              {status === "available" && t("update.available")}
              {status === "downloading" && (autoMode ? t("update.autoUpdating") : t("update.downloading"))}
              {status === "ready" && t("update.ready")}
              {status === "error" && t("update.error")}
            </h4>

            <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
              {status === "available" &&
                t("update.availableDescription", { version })}
              {status === "downloading" &&
                (autoMode
                  ? (contentLength > 0
                    ? t("update.autoUpdatingProgress", { progress: progressPercent })
                    : t("update.autoUpdatingPleaseWait"))
                  : (contentLength > 0
                    ? t("update.downloadingProgress", { progress: progressPercent })
                    : t("update.downloadingPleaseWait")))}
              {status === "ready" && t("update.readyDescription")}
              {status === "error" && (error || t("update.errorDescription"))}
            </p>

            {/* Progress bar */}
            {status === "downloading" && contentLength > 0 && (
              <div className="mt-2 w-full bg-gray-200 dark:bg-gray-700 rounded-full h-1.5">
                <div
                  className="bg-primary-600 h-1.5 rounded-full transition-all duration-300"
                  style={{ width: `${progressPercent}%` }}
                />
              </div>
            )}

            {/* Actions - only show manual buttons when NOT in auto mode */}
            {!autoMode && (
              <div className="flex items-center gap-2 mt-3">
                {status === "available" && (
                  <Button size="sm" onClick={downloadAndInstall}>
                    <Download className="h-3.5 w-3.5 mr-1.5" />
                    {t("update.downloadInstall")}
                  </Button>
                )}
                {status === "ready" && (
                  <Button size="sm" onClick={installAndRestart}>
                    <RefreshCw className="h-3.5 w-3.5 mr-1.5" />
                    {t("update.restartNow")}
                  </Button>
                )}
                {status === "error" && (
                  <Button size="sm" variant="secondary" onClick={checkForUpdate}>
                    <RefreshCw className="h-3.5 w-3.5 mr-1.5" />
                    {t("update.retry")}
                  </Button>
                )}
                {status !== "downloading" && (
                  <Button size="sm" variant="secondary" onClick={dismiss}>
                    {t("update.later")}
                  </Button>
                )}
              </div>
            )}

            {/* In auto mode with error, show retry */}
            {autoMode && status === "error" && (
              <div className="flex items-center gap-2 mt-3">
                <Button size="sm" variant="secondary" onClick={checkForUpdate}>
                  <RefreshCw className="h-3.5 w-3.5 mr-1.5" />
                  {t("update.retry")}
                </Button>
                <Button size="sm" variant="secondary" onClick={dismiss}>
                  {t("update.later")}
                </Button>
              </div>
            )}
          </div>

          {/* Close button - not shown during auto-mode download or manual download */}
          {status !== "downloading" && !autoMode && (
            <button
              onClick={dismiss}
              className="flex-shrink-0 p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
            >
              <X className="h-4 w-4" />
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
