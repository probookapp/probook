import { useTranslation } from "react-i18next";
import { usePosStore } from "../stores/usePosStore";
import { formatCurrency } from "@/lib/utils";

const formatAmount = formatCurrency;

export function CartTotals() {
  const { t } = useTranslation("pos");
  const { getSubtotalHt, getTotalVat, getTotalTtc, getFinalAmount, discountPercent, discountAmount, getItemCount } =
    usePosStore();

  const subtotalHt = getSubtotalHt();
  const totalVat = getTotalVat();
  const totalTtc = getTotalTtc();
  const finalAmount = getFinalAmount();
  const itemCount = getItemCount();
  const hasDiscount = discountPercent > 0 || discountAmount > 0;

  return (
    <div className="border-t border-(--color-border-primary) p-4 bg-(--color-bg-secondary)/30 shrink-0">
      <div className="space-y-1 text-sm">
        <div className="flex justify-between text-(--color-text-secondary)">
          <span>
            {t("subtotalHt")} ({itemCount} {t("items")})
          </span>
          <span>{formatAmount(subtotalHt)}</span>
        </div>
        <div className="flex justify-between text-(--color-text-secondary)">
          <span>{t("vat")}</span>
          <span>{formatAmount(totalVat)}</span>
        </div>
        {hasDiscount && (
          <>
            <div className="flex justify-between">
              <span>{t("totalTtc")}</span>
              <span>{formatAmount(totalTtc)}</span>
            </div>
            <div className="flex justify-between text-green-600">
              <span>
                {t("discount")}
                {discountPercent > 0 && ` (${discountPercent}%)`}
              </span>
              <span>-{formatAmount(totalTtc - finalAmount)}</span>
            </div>
          </>
        )}
        <div className="flex justify-between text-xl font-bold pt-2 border-t border-(--color-border-primary)">
          <span>{t("total")}</span>
          <span>{formatAmount(finalAmount)}</span>
        </div>
      </div>
    </div>
  );
}
