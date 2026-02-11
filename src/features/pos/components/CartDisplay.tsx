import { useTranslation } from "react-i18next";
import { Trash2, Plus, Minus } from "lucide-react";
import { usePosStore } from "../stores/usePosStore";
import { formatCurrency } from "@/lib/utils";

const formatAmount = formatCurrency;

export function CartDisplay() {
  const { t } = useTranslation("pos");
  const { items, updateQuantity, removeItem } = usePosStore();

  if (items.length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center text-(--color-text-secondary)">
        <div className="text-center">
          <p className="text-lg">{t("emptyCart")}</p>
          <p className="text-sm">{t("scanOrSearch")}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-auto p-4">
      <table className="w-full">
        <thead className="sticky top-0 bg-(--color-bg-primary)">
          <tr className="text-left text-sm text-(--color-text-secondary) border-b border-(--color-border-primary)">
            <th className="pb-2 font-medium">{t("product")}</th>
            <th className="pb-2 font-medium text-center w-32">{t("quantity")}</th>
            <th className="pb-2 font-medium text-right w-24">{t("unitPrice")}</th>
            <th className="pb-2 font-medium text-right w-24">{t("total")}</th>
            <th className="pb-2 w-10"></th>
          </tr>
        </thead>
        <tbody>
          {items.map((item) => {
            const lineTotal =
              item.quantity *
              item.unitPriceHt *
              (1 - item.discountPercent / 100) *
              (1 + item.vatRate / 100);

            return (
              <tr key={item.id} className="border-b border-(--color-border-primary) hover:bg-(--color-bg-secondary)/50">
                <td className="py-3">
                  <div>
                    <p className="font-medium">{item.designation}</p>
                    {item.barcode && (
                      <p className="text-xs text-(--color-text-secondary)">
                        {item.barcode}
                      </p>
                    )}
                    {item.discountPercent > 0 && (
                      <p className="text-xs text-green-600">
                        -{item.discountPercent}%
                      </p>
                    )}
                  </div>
                </td>
                <td className="py-3">
                  <div className="flex items-center justify-center gap-2">
                    <button
                      onClick={() => updateQuantity(item.id, item.quantity - 1)}
                      className="p-1 rounded hover:bg-(--color-bg-secondary)"
                    >
                      <Minus className="h-4 w-4" />
                    </button>
                    <span className="w-8 text-center font-medium">
                      {item.quantity}
                    </span>
                    <button
                      onClick={() => updateQuantity(item.id, item.quantity + 1)}
                      className="p-1 rounded hover:bg-(--color-bg-secondary)"
                    >
                      <Plus className="h-4 w-4" />
                    </button>
                  </div>
                </td>
                <td className="py-3 text-right">
                  {formatAmount(item.unitPriceHt * (1 + item.vatRate / 100))}
                </td>
                <td className="py-3 text-right font-medium">
                  {formatAmount(lineTotal)}
                </td>
                <td className="py-3">
                  <button
                    onClick={() => removeItem(item.id)}
                    className="p-1 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 rounded"
                  >
                    <Trash2 className="h-4 w-4" />
                  </button>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
