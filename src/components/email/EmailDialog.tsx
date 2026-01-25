import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { X, Send, Mail, User, FileText, Edit2 } from "lucide-react";
import { Button, Input, Textarea } from "@/components/ui";

interface EmailDialogProps {
  isOpen: boolean;
  onClose: () => void;
  recipientEmail: string;
  recipientName?: string;
  documentType: "invoice" | "quote" | "delivery_note";
  documentNumber: string;
  defaultSubject: string;
  defaultBody: string;
  companyName?: string;
}

export function EmailDialog({
  isOpen,
  onClose,
  recipientEmail,
  recipientName,
  documentType,
  documentNumber,
  defaultSubject,
  defaultBody,
  companyName,
}: EmailDialogProps) {
  const { t } = useTranslation("common");
  const [subject, setSubject] = useState(defaultSubject);
  const [body, setBody] = useState(defaultBody);
  const [isEditing, setIsEditing] = useState(false);

  useEffect(() => {
    setSubject(defaultSubject);
    setBody(defaultBody);
    setIsEditing(false);
  }, [defaultSubject, defaultBody, isOpen]);

  if (!isOpen) return null;

  const getDocumentTypeLabel = () => {
    switch (documentType) {
      case "invoice":
        return t("documents.invoice");
      case "quote":
        return t("documents.quote");
      case "delivery_note":
        return t("documents.deliveryNote");
    }
  };

  const handleSend = () => {
    const mailtoSubject = encodeURIComponent(subject);
    const mailtoBody = encodeURIComponent(body);
    window.open(`mailto:${recipientEmail}?subject=${mailtoSubject}&body=${mailtoBody}`);
    onClose();
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/50"
        onClick={onClose}
      />

      {/* Dialog */}
      <div className="relative bg-white rounded-lg shadow-xl w-full max-w-2xl mx-4 max-h-[90vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b bg-gray-50">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-primary-100 rounded-lg">
              <Mail className="h-5 w-5 text-primary-600" />
            </div>
            <div>
              <h2 className="text-lg font-semibold">{t("email.sendByEmail")}</h2>
              <p className="text-sm text-gray-500">
                {getDocumentTypeLabel()} {documentNumber}
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-200 rounded-lg transition-colors"
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6 space-y-6">
          {/* Recipient Info */}
          <div className="flex items-center gap-4 p-4 bg-gray-50 rounded-lg">
            <div className="p-2 bg-gray-200 rounded-full">
              <User className="h-5 w-5 text-gray-600" />
            </div>
            <div>
              <p className="font-medium">{recipientName || t("labels.client")}</p>
              <p className="text-sm text-gray-500">{recipientEmail}</p>
            </div>
          </div>

          {/* Document Info */}
          <div className="flex items-center gap-4 p-4 border rounded-lg">
            <div className="p-2 bg-blue-100 rounded-lg">
              <FileText className="h-5 w-5 text-blue-600" />
            </div>
            <div className="flex-1">
              <p className="font-medium">{getDocumentTypeLabel()} {documentNumber}</p>
              <p className="text-sm text-gray-500">
                {t("email.pdfAutoAttached")}
              </p>
            </div>
          </div>

          {/* Edit Toggle */}
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium text-gray-700">{t("email.message")}</span>
            <button
              onClick={() => setIsEditing(!isEditing)}
              className="flex items-center gap-1 text-sm text-primary-600 hover:text-primary-700"
            >
              <Edit2 className="h-4 w-4" />
              {isEditing ? t("email.preview") : t("email.editMessage")}
            </button>
          </div>

          {isEditing ? (
            <div className="space-y-4">
              <Input
                label={t("email.subject")}
                value={subject}
                onChange={(e) => setSubject(e.target.value)}
              />
              <Textarea
                label={t("email.body")}
                value={body}
                onChange={(e) => setBody(e.target.value)}
                className="min-h-50"
              />
            </div>
          ) : (
            <div className="border rounded-lg overflow-hidden">
              {/* Email Preview */}
              <div className="bg-gray-50 px-4 py-2 border-b">
                <p className="text-sm">
                  <span className="text-gray-500">{t("email.subjectLabel")}</span>{" "}
                  <span className="font-medium">{subject}</span>
                </p>
              </div>
              <div className="p-4 bg-white">
                <pre className="whitespace-pre-wrap font-sans text-sm text-gray-700">
                  {body}
                </pre>
              </div>
              {companyName && (
                <div className="bg-gray-50 px-4 py-2 border-t">
                  <p className="text-xs text-gray-500">
                    {t("email.sentFrom")} {companyName}
                  </p>
                </div>
              )}
            </div>
          )}

          {/* Hint */}
          <div className="flex items-start gap-3 p-4 bg-amber-50 rounded-lg">
            <Mail className="h-5 w-5 text-amber-600 mt-0.5" />
            <div>
              <p className="text-sm text-amber-800">
                <strong>Note:</strong> {t("email.note")}
              </p>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-end gap-3 px-6 py-4 border-t bg-gray-50">
          <Button variant="secondary" onClick={onClose}>
            {t("buttons.cancel")}
          </Button>
          <Button onClick={handleSend}>
            <Send className="h-4 w-4 mr-2" />
            {t("email.send")}
          </Button>
        </div>
      </div>
    </div>
  );
}
