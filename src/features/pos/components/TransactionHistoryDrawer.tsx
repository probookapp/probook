import { useState } from "react";
import { useTranslation } from "react-i18next";
import { X, Ban, ChevronDown, ChevronUp } from "lucide-react";
import { formatCurrency } from "@/lib/utils";
import {
  useSessionTransactions,
  useCancelTransaction,
} from "../hooks/usePosTransaction";
import type { PosTransaction } from "@/types";
import { toast } from "@/stores/useToastStore";

const formatAmount = formatCurrency;

interface TransactionHistoryDrawerProps {
  open: boolean;
  onClose: () => void;
  sessionId: string;
}

function TransactionRow({ tx }: { tx: PosTransaction }) {
  const { t } = useTranslation("pos");
  const [expanded, setExpanded] = useState(false);
  const cancelTransaction = useCancelTransaction();

  const isCancelled = tx.status === "CANCELLED";
  const time = new Date(tx.transaction_date).toLocaleTimeString([], {
    hour: "2-digit",
    minute: "2-digit",
  });

  const handleCancel = async () => {
    if (!confirm(t("confirmCancelTransaction"))) return;
    try {
      await cancelTransaction.mutateAsync({
        id: tx.id,
        reason: t("cancelledByUser"),
      });
      toast.success(t("transactionCancelled"));
    } catch {
      toast.error(t("errors.cancelFailed"));
    }
  };

  return (
    <div
      className={`border border-(--color-border-primary) rounded-lg overflow-hidden ${
        isCancelled ? "opacity-60" : ""
      }`}
    >
      {/* Summary row */}
      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full flex items-center justify-between p-3 hover:bg-(--color-bg-secondary) transition-colors text-left"
      >
        <div className="flex items-center gap-3 min-w-0">
          <div>
            <p className="font-medium text-sm">{tx.ticket_number}</p>
            <p className="text-xs text-(--color-text-secondary)">{time}</p>
          </div>
          {isCancelled && (
            <span className="text-xs px-2 py-0.5 bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400 rounded-full">
              {t("cancelled")}
            </span>
          )}
        </div>
        <div className="flex items-center gap-2">
          <span className={`font-bold ${isCancelled ? "line-through" : ""}`}>
            {formatAmount(tx.final_amount)}
          </span>
          {expanded ? (
            <ChevronUp className="h-4 w-4 text-(--color-text-secondary)" />
          ) : (
            <ChevronDown className="h-4 w-4 text-(--color-text-secondary)" />
          )}
        </div>
      </button>

      {/* Expanded details */}
      {expanded && (
        <div className="border-t border-(--color-border-primary) p-3 bg-(--color-bg-secondary)/50 space-y-2">
          {/* Line items */}
          <div className="space-y-1">
            {tx.lines.map((line) => (
              <div
                key={line.id}
                className="flex justify-between text-sm"
              >
                <span className="text-(--color-text-secondary)">
                  {line.quantity}x {line.designation}
                </span>
                <span>{formatAmount(line.total_ttc)}</span>
              </div>
            ))}
          </div>

          {/* Payments */}
          <div className="border-t border-(--color-border-primary) pt-2 space-y-1">
            {tx.payments.map((payment) => (
              <div
                key={payment.id}
                className="flex justify-between text-sm"
              >
                <span className="text-(--color-text-secondary)">
                  {payment.payment_method === "CASH"
                    ? t("cash")
                    : t("card")}
                </span>
                <span>{formatAmount(payment.amount)}</span>
              </div>
            ))}
            {tx.payments.some(
              (p) =>
                p.payment_method === "CASH" &&
                p.change_given &&
                p.change_given > 0
            ) && (
              <div className="flex justify-between text-sm text-(--color-text-secondary)">
                <span>{t("change")}</span>
                <span>
                  {formatAmount(
                    tx.payments.find(
                      (p) => p.payment_method === "CASH" && p.change_given
                    )?.change_given ?? 0
                  )}
                </span>
              </div>
            )}
          </div>

          {/* Cancel button */}
          {!isCancelled && (
            <button
              onClick={handleCancel}
              disabled={cancelTransaction.isPending}
              className="w-full mt-2 px-3 py-2 text-sm text-red-600 dark:text-red-400 border border-red-300 dark:border-red-800 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 flex items-center justify-center gap-2 transition-colors disabled:opacity-50"
            >
              <Ban className="h-3.5 w-3.5" />
              {cancelTransaction.isPending
                ? t("loading")
                : t("cancelTransaction")}
            </button>
          )}

          {/* Cancel reason */}
          {isCancelled && tx.notes && (
            <p className="text-xs text-(--color-text-secondary) italic">
              {tx.notes}
            </p>
          )}
        </div>
      )}
    </div>
  );
}

export function TransactionHistoryDrawer({
  open,
  onClose,
  sessionId,
}: TransactionHistoryDrawerProps) {
  const { t } = useTranslation("pos");
  const { data: transactions, isLoading } =
    useSessionTransactions(open ? sessionId : undefined);

  if (!open) return null;

  const completedTotal =
    transactions
      ?.filter((tx) => tx.status === "COMPLETED")
      .reduce((sum, tx) => sum + tx.final_amount, 0) ?? 0;

  return (
    <div className="fixed inset-0 z-50 flex">
      {/* Backdrop */}
      <div className="flex-1 bg-black/50" onClick={onClose} />

      {/* Drawer */}
      <div className="w-full max-w-md bg-(--color-bg-primary) shadow-xl flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-(--color-border-primary) shrink-0">
          <div>
            <h2 className="text-lg font-bold">{t("transactionHistory")}</h2>
            {transactions && (
              <p className="text-sm text-(--color-text-secondary)">
                {transactions.length} {t("transactions").toLowerCase()} &middot;{" "}
                {formatAmount(completedTotal)}
              </p>
            )}
          </div>
          <button
            onClick={onClose}
            className="p-1 hover:bg-(--color-bg-secondary) rounded"
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-auto p-4">
          {isLoading ? (
            <div className="text-center py-8 text-(--color-text-secondary)">
              {t("loading")}
            </div>
          ) : !transactions || transactions.length === 0 ? (
            <div className="text-center py-8 text-(--color-text-secondary)">
              {t("noTransactions")}
            </div>
          ) : (
            <div className="space-y-2">
              {transactions.map((tx) => (
                <TransactionRow key={tx.id} tx={tx} />
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
