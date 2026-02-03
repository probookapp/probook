import { useState, useMemo, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { pdf } from "@react-pdf/renderer";
import { toast } from "@/stores/useToastStore";
import { Download, Eye, Loader2 } from "lucide-react";
import { Button } from "@/components/ui";
import { InvoicePDF } from "./InvoicePDF";
import { QuotePDF } from "./QuotePDF";
import { DeliveryNotePDF } from "./DeliveryNotePDF";
import { useLogoBase64 } from "@/features/settings";
import { isTauri } from "@/lib/config";
import type { Invoice, Quote, DeliveryNote, CompanySettings } from "@/types";

interface InvoicePDFViewerProps {
  type: "invoice";
  document: Invoice;
  company: CompanySettings;
}

interface QuotePDFViewerProps {
  type: "quote";
  document: Quote;
  company: CompanySettings;
}

interface DeliveryNotePDFViewerProps {
  type: "delivery_note";
  document: DeliveryNote;
  company: CompanySettings;
  logoBase64?: string | null;
}

type PDFViewerProps = InvoicePDFViewerProps | QuotePDFViewerProps | DeliveryNotePDFViewerProps;

export function PDFViewer(props: PDFViewerProps) {
  const { type, document: doc, company } = props;
  const { t } = useTranslation("common");
  const [isOpening, setIsOpening] = useState(false);
  const [isDownloading, setIsDownloading] = useState(false);
  const { data: currentLogo, isLoading: isLoadingLogo } = useLogoBase64();

  // Get filename based on document type
  const fileName = useMemo(() => {
    if (type === "invoice") {
      return `${(doc as Invoice).invoice_number}.pdf`;
    } else if (type === "quote") {
      return `${(doc as Quote).quote_number}.pdf`;
    } else {
      return `${(doc as DeliveryNote).delivery_note_number}.pdf`;
    }
  }, [type, doc]);

  // Determine which logo to use:
  // - For invoices: use stored logo_snapshot if issued/paid, otherwise use current logo
  // - For quotes: use stored logo_snapshot if sent/accepted, otherwise use current logo
  // - For delivery notes: use passed logoBase64 or current logo
  const logoToUse = useMemo(() => {
    if (type === "invoice") {
      const invoice = doc as Invoice;
      // Use stored snapshot for issued/paid invoices, current logo for drafts
      if (invoice.status !== "DRAFT" && invoice.logo_snapshot) {
        return invoice.logo_snapshot;
      }
    } else if (type === "quote") {
      const quote = doc as Quote;
      // Use stored snapshot for sent/accepted quotes, current logo for drafts
      if (quote.status !== "DRAFT" && quote.logo_snapshot) {
        return quote.logo_snapshot;
      }
    } else if (type === "delivery_note") {
      // Use passed logo or current logo for delivery notes
      const passedLogo = (props as DeliveryNotePDFViewerProps).logoBase64;
      if (passedLogo !== undefined) {
        return passedLogo;
      }
    }
    return currentLogo;
  }, [type, doc, currentLogo, props]);

  // Memoize the PDF document
  const PDFDocument = useMemo(() => {
    if (type === "invoice") {
      return <InvoicePDF invoice={doc as Invoice} company={company} logoBase64={logoToUse} />;
    } else if (type === "quote") {
      return <QuotePDF quote={doc as Quote} company={company} logoBase64={logoToUse} />;
    } else {
      return <DeliveryNotePDF deliveryNote={doc as DeliveryNote} company={company} logoBase64={logoToUse} />;
    }
  }, [type, doc, company, logoToUse]);

  // Open PDF in a new window for preview/print
  const handleOpenPreview = useCallback(async () => {
    setIsOpening(true);
    try {
      const blob = await pdf(PDFDocument).toBlob();
      const url = URL.createObjectURL(blob);

      if (isTauri()) {
        const { WebviewWindow } = await import("@tauri-apps/api/webviewWindow");
        const windowLabel = `pdf-preview-${Date.now()}`;
        const previewWindow = new WebviewWindow(windowLabel, {
          url: url,
          title: fileName,
          width: 900,
          height: 700,
          center: true,
          resizable: true,
        });

        previewWindow.once("tauri://destroyed", () => {
          URL.revokeObjectURL(url);
        });

        previewWindow.once("tauri://error", () => {
          toast.error(t("pdfViewer.errorPreview"));
          URL.revokeObjectURL(url);
        });
      } else {
        // Web mode: open PDF in a new browser tab
        window.open(url, "_blank");
      }
    } catch (error) {
      toast.error(t("pdfViewer.errorPreview"));
    } finally {
      setIsOpening(false);
    }
  }, [PDFDocument, fileName, t]);

  // Download PDF with save dialog (Tauri) or browser download (web)
  const handleDownload = useCallback(async () => {
    setIsDownloading(true);
    try {
      const blob = await pdf(PDFDocument).toBlob();

      if (isTauri()) {
        const { save } = await import("@tauri-apps/plugin-dialog");
        const { writeFile } = await import("@tauri-apps/plugin-fs");
        const arrayBuffer = await blob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        const filePath = await save({
          defaultPath: fileName,
          filters: [{ name: "PDF", extensions: ["pdf"] }],
        });

        if (filePath) {
          await writeFile(filePath, uint8Array);
          toast.success(t("pdfViewer.downloadSuccess", { path: filePath }));
        }
      } else {
        // Web mode: trigger browser download
        const url = URL.createObjectURL(blob);
        const link = document.createElement("a");
        link.href = url;
        link.download = fileName;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);
        toast.success(t("pdfViewer.downloadSuccess", { path: fileName }));
      }
    } catch (error) {
      toast.error(t("pdfViewer.downloadError"));
    } finally {
      setIsDownloading(false);
    }
  }, [PDFDocument, fileName, t]);

  // Only wait for logo loading if we're showing a draft document or delivery note
  const needsCurrentLogo =
    (type === "invoice" && (doc as Invoice).status === "DRAFT") ||
    (type === "quote" && (doc as Quote).status === "DRAFT") ||
    (type === "delivery_note" && !(props as DeliveryNotePDFViewerProps).logoBase64);
  if (needsCurrentLogo && isLoadingLogo) {
    return (
      <div className="flex items-center gap-2 text-gray-500">
        <Loader2 className="h-4 w-4 animate-spin" />
        {t("pdfViewer.loading")}
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-2">
      <Button
        size="sm"
        className="w-full justify-center"
        onClick={handleDownload}
        disabled={isDownloading}
      >
        {isDownloading ? (
          <Loader2 className="h-4 w-4 mr-2 animate-spin" />
        ) : (
          <Download className="h-4 w-4 mr-2" />
        )}
        {t("pdfViewer.downloadPdf")}
      </Button>

      <Button
        variant="secondary"
        size="sm"
        className="w-full justify-center"
        onClick={handleOpenPreview}
        disabled={isOpening}
      >
        {isOpening ? (
          <Loader2 className="h-4 w-4 mr-2 animate-spin" />
        ) : (
          <Eye className="h-4 w-4 mr-2" />
        )}
        {t("pdfViewer.preview")}
      </Button>
    </div>
  );
}
