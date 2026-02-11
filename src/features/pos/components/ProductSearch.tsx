import { useState, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Search } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { productApi } from "@/lib/tauri";
import type { Product } from "@/types";
import { formatCurrency } from "@/lib/utils";

const formatAmount = formatCurrency;

interface ProductSearchProps {
  onProductSelect: (product: Product) => void;
}

export function ProductSearch({ onProductSelect }: ProductSearchProps) {
  const { t } = useTranslation("pos");
  const [searchTerm, setSearchTerm] = useState("");

  const { data: products } = useQuery({
    queryKey: ["products"],
    queryFn: productApi.getAll,
  });

  const filteredProducts = useMemo(() => {
    if (!products) return [];
    if (!searchTerm) return products.slice(0, 20);

    const term = searchTerm.toLowerCase();
    return products
      .filter(
        (p) =>
          p.designation.toLowerCase().includes(term) ||
          p.reference?.toLowerCase().includes(term) ||
          p.barcode?.toLowerCase().includes(term)
      )
      .slice(0, 20);
  }, [products, searchTerm]);

  return (
    <div className="flex flex-col h-full">
      {/* Search input */}
      <div className="p-4 border-b border-(--color-border-primary)">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-(--color-text-secondary)" />
          <input
            type="text"
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            placeholder={t("searchProducts")}
            className="w-full pl-10 pr-4 py-3 border border-(--color-border-input) rounded-lg bg-(--color-bg-input) focus:outline-none focus:ring-2 focus:ring-primary-500"
            data-barcode-input="true"
          />
        </div>
      </div>

      {/* Product grid */}
      <div className="flex-1 overflow-auto p-4">
        <div className="grid grid-cols-2 gap-2">
          {filteredProducts.map((product) => (
            <button
              key={product.id}
              onClick={() => onProductSelect(product)}
              className="p-3 text-left border border-(--color-border-primary) rounded-lg hover:bg-(--color-bg-secondary) transition-colors"
            >
              <p className="font-medium truncate">{product.designation}</p>
              <div className="flex justify-between items-center mt-1">
                <span className="text-xs text-(--color-text-secondary)">
                  {product.reference || product.barcode || "-"}
                </span>
                <span className="font-bold text-primary-600">
                  {formatAmount(product.unit_price_ht * (1 + product.vat_rate / 100))}
                </span>
              </div>
              {product.quantity !== null && product.quantity <= 5 && !product.is_service && (
                <p className={`text-xs mt-1 ${product.quantity === 0 ? "text-red-600 dark:text-red-400" : "text-orange-500 dark:text-orange-400"}`}>
                  {product.quantity === 0
                    ? t("outOfStock")
                    : t("lowStock", { count: product.quantity })}
                </p>
              )}
            </button>
          ))}
        </div>

        {filteredProducts.length === 0 && (
          <div className="text-center text-(--color-text-secondary) py-8">
            {searchTerm ? t("noProductsFound") : t("noProducts")}
          </div>
        )}
      </div>
    </div>
  );
}
