import { useTranslation } from "react-i18next";
import { Modal, Button } from "@/components/ui";
import type { Blocker } from "react-router-dom";

interface UnsavedChangesDialogProps {
  blocker: Blocker;
}

export function UnsavedChangesDialog({ blocker }: UnsavedChangesDialogProps) {
  const { t } = useTranslation("common");

  if (blocker.state !== "blocked") return null;

  return (
    <Modal
      isOpen
      onClose={() => blocker.reset()}
      title={t("unsavedChanges.title")}
      size="sm"
    >
      <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
        {t("unsavedChanges.message")}
      </p>
      <div className="flex justify-end gap-3">
        <Button variant="secondary" size="sm" onClick={() => blocker.reset()}>
          {t("unsavedChanges.stay")}
        </Button>
        <Button
          size="sm"
          onClick={() => blocker.proceed()}
          className="bg-red-600 hover:bg-red-700 text-white"
        >
          {t("unsavedChanges.leave")}
        </Button>
      </div>
    </Modal>
  );
}
