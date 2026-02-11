import { useState } from "react";
import { useTranslation } from "react-i18next";
import { X, AlertCircle } from "lucide-react";
import { formatCurrency } from "@/lib/utils";
import { useSettingsStore } from "@/stores/useSettingsStore";
import { useSessionSummary } from "../hooks/usePosSession";

const formatAmount = formatCurrency;

interface CloseSessionModalProps {
  open: boolean;
  onClose: () => void;
  onConfirm: (actualCash: number, notes?: string) => void;
  sessionId: string;
  isLoading: boolean;
}

export function CloseSessionModal({
  open,
  onClose,
  onConfirm,
  sessionId,
  isLoading,
}: CloseSessionModalProps) {
  const { t } = useTranslation();
  const currency = useSettingsStore((state) => state.currency);
  const [actualCash, setActualCash] = useState<string>("");
  const [notes, setNotes] = useState<string>("");

  const { data: summary, isLoading: summaryLoading } = useSessionSummary(
    open ? sessionId : undefined
  );

  if (!open) return null;

  const handleConfirm = () => {
    const amount = parseFloat(actualCash) || 0;
    onConfirm(amount, notes || undefined);
  };

  const expectedCash = summary
    ? summary.session.opening_float + summary.cash_sales + summary.net_cash_movement
    : 0;

  const actualAmount = parseFloat(actualCash) || 0;
  const difference = actualAmount - expectedCash;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-background rounded-xl shadow-xl w-full max-w-lg mx-4 max-h-[90vh] overflow-auto">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b sticky top-0 bg-background">
          <h2 className="text-xl font-bold">{t("pos.closeSession")}</h2>
          <button onClick={onClose} className="p-1 hover:bg-muted rounded">
            <X className="h-5 w-5" />
          </button>
        </div>

        {/* Content */}
        <div className="p-4 space-y-6">
          {summaryLoading ? (
            <div className="text-center py-8 text-muted-foreground">
              {t("common.loading")}...
            </div>
          ) : summary ? (
            <>
              {/* Session summary */}
              <div className="space-y-2 text-sm">
                <h3 className="font-bold text-lg">{t("pos.sessionSummary")}</h3>
                <div className="grid grid-cols-2 gap-2">
                  <div className="p-3 bg-muted rounded-lg">
                    <p className="text-muted-foreground">{t("pos.transactions")}</p>
                    <p className="text-2xl font-bold">{summary.transaction_count}</p>
                  </div>
                  <div className="p-3 bg-muted rounded-lg">
                    <p className="text-muted-foreground">{t("pos.totalSales")}</p>
                    <p className="text-2xl font-bold">{formatAmount(summary.total_sales)}</p>
                  </div>
                  <div className="p-3 bg-muted rounded-lg">
                    <p className="text-muted-foreground">{t("pos.cashSales")}</p>
                    <p className="text-xl font-bold">{formatAmount(summary.cash_sales)}</p>
                  </div>
                  <div className="p-3 bg-muted rounded-lg">
                    <p className="text-muted-foreground">{t("pos.cardSales")}</p>
                    <p className="text-xl font-bold">{formatAmount(summary.card_sales)}</p>
                  </div>
                </div>

                {/* Expected cash breakdown */}
                <div className="mt-4 p-3 border rounded-lg space-y-1">
                  <div className="flex justify-between">
                    <span>{t("pos.openingFloat")}</span>
                    <span>{formatAmount(summary.session.opening_float)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span>{t("pos.cashSales")}</span>
                    <span>+{formatAmount(summary.cash_sales)}</span>
                  </div>
                  {summary.net_cash_movement !== 0 && (
                    <div className="flex justify-between">
                      <span>{t("pos.cashMovements")}</span>
                      <span>
                        {summary.net_cash_movement >= 0 ? "+" : ""}
                        {formatAmount(summary.net_cash_movement)}
                      </span>
                    </div>
                  )}
                  <div className="flex justify-between font-bold border-t pt-1">
                    <span>{t("pos.expectedCash")}</span>
                    <span>{formatAmount(expectedCash)}</span>
                  </div>
                </div>
              </div>

              {/* Cash count */}
              <div>
                <label className="block text-sm font-medium mb-1">
                  {t("pos.actualCash")} ({currency})
                </label>
                <input
                  type="number"
                  value={actualCash}
                  onChange={(e) => setActualCash(e.target.value)}
                  className="w-full px-4 py-3 border rounded-lg text-2xl text-center font-bold focus:outline-none focus:ring-2 focus:ring-primary"
                  placeholder="0.00"
                  min="0"
                  step="0.01"
                  autoFocus
                />
              </div>

              {/* Difference */}
              {actualCash && (
                <div
                  className={`p-4 rounded-lg ${
                    Math.abs(difference) < 0.01
                      ? "bg-green-100 dark:bg-green-900/30"
                      : difference < 0
                      ? "bg-red-100 dark:bg-red-900/30"
                      : "bg-orange-100 dark:bg-orange-900/30"
                  }`}
                >
                  <div className="flex items-center gap-2">
                    {Math.abs(difference) >= 0.01 && (
                      <AlertCircle className="h-5 w-5 shrink-0" />
                    )}
                    <div>
                      <p className="text-sm font-medium">
                        {Math.abs(difference) < 0.01
                          ? t("pos.cashBalanced")
                          : difference < 0
                          ? t("pos.cashShort")
                          : t("pos.cashOver")}
                      </p>
                      {Math.abs(difference) >= 0.01 && (
                        <p className="text-2xl font-bold">
                          {formatAmount(Math.abs(difference))}
                        </p>
                      )}
                    </div>
                  </div>
                </div>
              )}

              {/* Notes */}
              <div>
                <label className="block text-sm font-medium mb-1">
                  {t("pos.notes")} ({t("common.optional")})
                </label>
                <textarea
                  value={notes}
                  onChange={(e) => setNotes(e.target.value)}
                  className="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                  rows={2}
                  placeholder={t("pos.closeNotesPlaceholder")}
                />
              </div>
            </>
          ) : null}
        </div>

        {/* Footer */}
        <div className="p-4 border-t flex gap-3 sticky bottom-0 bg-background">
          <button
            onClick={onClose}
            className="flex-1 px-4 py-3 border rounded-lg hover:bg-muted font-medium"
          >
            {t("common.cancel")}
          </button>
          <button
            onClick={handleConfirm}
            disabled={isLoading || !actualCash}
            className="flex-1 px-4 py-3 bg-primary hover:bg-primary/90 text-primary-foreground rounded-lg font-bold disabled:opacity-50"
          >
            {isLoading ? t("common.loading") : t("pos.closeSession")}
          </button>
        </div>
      </div>
    </div>
  );
}
