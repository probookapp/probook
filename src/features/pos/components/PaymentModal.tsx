import { useState } from "react";
import { useTranslation } from "react-i18next";
import { X, Banknote, CreditCard } from "lucide-react";
import { formatCurrency } from "@/lib/utils";

const formatAmount = formatCurrency;

interface PaymentModalProps {
  open: boolean;
  onClose: () => void;
  onConfirm: (payments: Array<{ method: string; amount: number; cashGiven?: number }>) => void;
  totalAmount: number;
  isLoading: boolean;
}

export function PaymentModal({
  open,
  onClose,
  onConfirm,
  totalAmount,
  isLoading,
}: PaymentModalProps) {
  const { t } = useTranslation("pos");
  const [paymentMethod, setPaymentMethod] = useState<"CASH" | "CARD">("CASH");
  const [cashGiven, setCashGiven] = useState<string>("");

  if (!open) return null;

  const cashAmount = parseFloat(cashGiven) || 0;
  const change = paymentMethod === "CASH" ? cashAmount - totalAmount : 0;
  const isValid =
    paymentMethod === "CARD" || (paymentMethod === "CASH" && cashAmount >= totalAmount);

  const handleConfirm = () => {
    const payments = [
      {
        method: paymentMethod,
        amount: totalAmount,
        cashGiven: paymentMethod === "CASH" ? cashAmount : undefined,
      },
    ];
    onConfirm(payments);
  };

  const quickAmounts = [
    Math.ceil(totalAmount / 10) * 10,
    Math.ceil(totalAmount / 50) * 50,
    Math.ceil(totalAmount / 100) * 100,
    Math.ceil(totalAmount / 500) * 500,
  ].filter((v, i, a) => a.indexOf(v) === i && v >= totalAmount);

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-background rounded-xl shadow-xl w-full max-w-md mx-4">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b">
          <h2 className="text-xl font-bold">{t("payment")}</h2>
          <button onClick={onClose} className="p-1 hover:bg-muted rounded">
            <X className="h-5 w-5" />
          </button>
        </div>

        {/* Content */}
        <div className="p-4 space-y-6">
          {/* Total */}
          <div className="text-center">
            <p className="text-sm text-muted-foreground">{t("totalToPay")}</p>
            <p className="text-4xl font-bold">{formatAmount(totalAmount)}</p>
          </div>

          {/* Payment method selection */}
          <div className="grid grid-cols-2 gap-3">
            <button
              onClick={() => setPaymentMethod("CASH")}
              className={`p-4 rounded-lg border-2 flex flex-col items-center gap-2 transition-colors ${
                paymentMethod === "CASH"
                  ? "border-primary bg-primary/10"
                  : "border-muted hover:border-muted-foreground"
              }`}
            >
              <Banknote className="h-8 w-8" />
              <span className="font-medium">{t("cash")}</span>
            </button>
            <button
              onClick={() => setPaymentMethod("CARD")}
              className={`p-4 rounded-lg border-2 flex flex-col items-center gap-2 transition-colors ${
                paymentMethod === "CARD"
                  ? "border-primary bg-primary/10"
                  : "border-muted hover:border-muted-foreground"
              }`}
            >
              <CreditCard className="h-8 w-8" />
              <span className="font-medium">{t("card")}</span>
            </button>
          </div>

          {/* Cash input */}
          {paymentMethod === "CASH" && (
            <div className="space-y-3">
              <div>
                <label className="block text-sm font-medium mb-1">
                  {t("cashGiven")}
                </label>
                <input
                  type="number"
                  value={cashGiven}
                  onChange={(e) => setCashGiven(e.target.value)}
                  className="w-full px-4 py-3 border rounded-lg text-2xl text-center font-bold focus:outline-none focus:ring-2 focus:ring-primary"
                  placeholder="0.00"
                  autoFocus
                />
              </div>

              {/* Quick amounts */}
              <div className="flex gap-2 flex-wrap">
                {quickAmounts.slice(0, 4).map((amount) => (
                  <button
                    key={amount}
                    onClick={() => setCashGiven(amount.toString())}
                    className="px-4 py-2 border rounded-lg hover:bg-muted text-sm font-medium"
                  >
                    {formatAmount(amount)}
                  </button>
                ))}
              </div>

              {/* Change */}
              {cashAmount >= totalAmount && (
                <div className="p-4 bg-green-100 dark:bg-green-900/30 rounded-lg text-center">
                  <p className="text-sm text-green-700 dark:text-green-300">
                    {t("change")}
                  </p>
                  <p className="text-3xl font-bold text-green-700 dark:text-green-300">
                    {formatAmount(change)}
                  </p>
                </div>
              )}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="p-4 border-t flex gap-3">
          <button
            onClick={onClose}
            className="flex-1 px-4 py-3 border rounded-lg hover:bg-muted font-medium"
          >
            {t("cancel")}
          </button>
          <button
            onClick={handleConfirm}
            disabled={!isValid || isLoading}
            className="flex-1 px-4 py-3 bg-green-600 hover:bg-green-700 text-white rounded-lg font-bold disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isLoading ? t("loading") : t("confirm")}
          </button>
        </div>
      </div>
    </div>
  );
}
