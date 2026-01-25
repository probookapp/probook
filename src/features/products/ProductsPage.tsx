import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Plus, Pencil, Trash2, Search, Package, Briefcase, Folder, Tags } from "lucide-react";
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Modal,
  Table,
  TableHeader,
  TableBody,
  TableRow,
  TableHead,
  TableCell,
  Input,
  Badge,
} from "@/components/ui";
import { ProductForm } from "./components/ProductForm";
import { CategoryManager } from "./components/CategoryManager";
import {
  useProducts,
  useCreateProduct,
  useUpdateProduct,
  useDeleteProduct,
} from "./hooks/useProducts";
import { useProductCategories } from "./hooks/useProductCategories";
import { formatCurrency } from "@/lib/utils";
import type { Product } from "@/types";
import type { ProductFormData } from "./schemas/productSchema";

type TabType = "products" | "categories";

export function ProductsPage() {
  const { t } = useTranslation("products");
  const { t: tCommon } = useTranslation("common");
  const [activeTab, setActiveTab] = useState<TabType>("products");
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedProduct, setSelectedProduct] = useState<Product | undefined>();
  const [searchQuery, setSearchQuery] = useState("");
  const [deleteConfirmId, setDeleteConfirmId] = useState<string | null>(null);
  const [categoryFilter, setCategoryFilter] = useState<string>("");

  const { data: products, isLoading } = useProducts();
  const { data: categories } = useProductCategories();
  const createProduct = useCreateProduct();
  const updateProduct = useUpdateProduct();
  const deleteProduct = useDeleteProduct();

  const getCategoryName = (categoryId: string | null) => {
    if (!categoryId || !categories) return null;
    return categories.find((c) => c.id === categoryId)?.name || null;
  };

  const filteredProducts = products?.filter((product) => {
    const matchesSearch =
      product.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      product.reference?.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesCategory = !categoryFilter || product.category_id === categoryFilter;
    return matchesSearch && matchesCategory;
  });

  const handleOpenModal = (product?: Product) => {
    setSelectedProduct(product);
    setIsModalOpen(true);
  };

  const handleCloseModal = () => {
    setSelectedProduct(undefined);
    setIsModalOpen(false);
  };

  const handleSubmit = async (data: ProductFormData) => {
    // Transform empty strings to null for optional fields
    const input = {
      ...data,
      description: data.description || null,
      reference: data.reference || null,
      category_id: data.category_id || null,
    };

    if (selectedProduct) {
      await updateProduct.mutateAsync({ ...input, id: selectedProduct.id });
    } else {
      await createProduct.mutateAsync(input);
    }
    handleCloseModal();
  };

  const handleDelete = async (id: string) => {
    await deleteProduct.mutateAsync(id);
    setDeleteConfirmId(null);
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600" />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl sm:text-2xl font-bold text-gray-900 dark:text-gray-100">{t("title")}</h1>
          <p className="text-sm sm:text-base text-gray-500 dark:text-gray-400">{t("subtitle")}</p>
        </div>
        {activeTab === "products" && (
          <Button onClick={() => handleOpenModal()} size="sm" className="self-start sm:self-auto">
            <Plus className="h-4 w-4 mr-2" />
            {t("newProduct")}
          </Button>
        )}
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200 dark:border-gray-700">
        <nav className="-mb-px flex gap-4 sm:gap-6">
          <button
            onClick={() => setActiveTab("products")}
            className={`flex items-center gap-2 py-3 px-1 border-b-2 text-sm font-medium transition-colors ${
              activeTab === "products"
                ? "border-primary-500 text-primary-600 dark:text-primary-400"
                : "border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300"
            }`}
          >
            <Package className="h-4 w-4" />
            {t("tabs.products")} ({products?.length || 0})
          </button>
          <button
            onClick={() => setActiveTab("categories")}
            className={`flex items-center gap-2 py-3 px-1 border-b-2 text-sm font-medium transition-colors ${
              activeTab === "categories"
                ? "border-primary-500 text-primary-600 dark:text-primary-400"
                : "border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300"
            }`}
          >
            <Tags className="h-4 w-4" />
            {t("tabs.categories")} ({categories?.length || 0})
          </button>
        </nav>
      </div>

      {activeTab === "categories" ? (
        <CategoryManager />
      ) : (
        <Card>
          <CardHeader>
            <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
              <CardTitle>{t("productList")}</CardTitle>
              <div className="flex flex-col gap-2 sm:flex-row sm:items-center sm:gap-3">
                {categories && categories.length > 0 && (
                  <select
                    id="category-filter"
                    name="category-filter"
                    value={categoryFilter}
                    onChange={(e) => setCategoryFilter(e.target.value)}
                    className="h-10 rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary-500"
                  >
                    <option value="">{t("allCategories")}</option>
                    {categories.map((cat) => (
                      <option key={cat.id} value={cat.id}>
                        {cat.name}
                      </option>
                    ))}
                  </select>
                )}
                <div className="relative w-full sm:w-56 md:w-64 lg:w-72">
                  <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
                  <Input
                    id="product-search"
                    name="product-search"
                    placeholder={t("searchPlaceholder")}
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    autoComplete="off"
                    className="pl-9"
                  />
                </div>
              </div>
            </div>
          </CardHeader>
          <CardContent className="p-0 overflow-x-auto">
            <Table className="min-w-200">
              <TableHeader>
                <TableRow>
                  <TableHead>{t("fields.type")}</TableHead>
                  <TableHead>{t("fields.reference")}</TableHead>
                  <TableHead>{t("fields.name")}</TableHead>
                  <TableHead>{t("fields.category")}</TableHead>
                  <TableHead>{t("fields.priceHT")}</TableHead>
                  <TableHead>{t("fields.vat")}</TableHead>
                  <TableHead>{t("fields.unit")}</TableHead>
                  <TableHead className="w-24">{tCommon("buttons.actions")}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {filteredProducts && filteredProducts.length > 0 ? (
                  filteredProducts.map((product) => (
                    <TableRow key={product.id}>
                      <TableCell>
                        {product.is_service ? (
                          <Badge variant="info">
                            <Briefcase className="h-3 w-3 mr-1" />
                            {t("types.service")}
                          </Badge>
                        ) : (
                          <Badge variant="default">
                            <Package className="h-3 w-3 mr-1" />
                            {t("types.product")}
                          </Badge>
                        )}
                      </TableCell>
                      <TableCell className="font-mono text-sm text-gray-600 dark:text-gray-400">
                        {product.reference || "-"}
                      </TableCell>
                      <TableCell className="font-medium text-gray-900 dark:text-gray-100">{product.name}</TableCell>
                      <TableCell>
                        {getCategoryName(product.category_id) ? (
                          <span className="inline-flex items-center gap-1 text-sm text-gray-600 dark:text-gray-400">
                            <Folder className="h-3 w-3 text-amber-500" />
                            {getCategoryName(product.category_id)}
                          </span>
                        ) : (
                          <span className="text-gray-400 dark:text-gray-500">-</span>
                        )}
                      </TableCell>
                      <TableCell className="text-gray-600 dark:text-gray-400">{formatCurrency(product.unit_price_ht)}</TableCell>
                      <TableCell className="text-gray-600 dark:text-gray-400">{product.vat_rate}%</TableCell>
                      <TableCell className="text-gray-600 dark:text-gray-400">{product.unit}</TableCell>
                      <TableCell>
                        <div className="flex items-center gap-2">
                          <button
                            onClick={() => handleOpenModal(product)}
                            aria-label={tCommon("buttons.edit")}
                            className="p-1 text-gray-500 hover:text-primary-600 dark:hover:text-primary-400 transition-colors"
                          >
                            <Pencil className="h-4 w-4" />
                          </button>
                          <button
                            onClick={() => setDeleteConfirmId(product.id)}
                            aria-label={tCommon("buttons.delete")}
                            className="p-1 text-gray-500 hover:text-red-600 transition-colors"
                          >
                            <Trash2 className="h-4 w-4" />
                          </button>
                        </div>
                      </TableCell>
                    </TableRow>
                  ))
                ) : (
                  <TableRow>
                    <TableCell colSpan={8} className="text-center text-gray-500 dark:text-gray-400 py-8">
                      {t("noProducts")}
                    </TableCell>
                  </TableRow>
                )}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      )}

      <Modal
        isOpen={isModalOpen}
        onClose={handleCloseModal}
        title={selectedProduct ? t("editProduct") : t("newProduct")}
        size="lg"
      >
        <ProductForm
          product={selectedProduct}
          onSubmit={handleSubmit}
          onCancel={handleCloseModal}
          isLoading={createProduct.isPending || updateProduct.isPending}
        />
      </Modal>

      <Modal
        isOpen={!!deleteConfirmId}
        onClose={() => setDeleteConfirmId(null)}
        title={tCommon("messages.confirmDelete")}
        size="sm"
      >
        <p className="text-gray-600 dark:text-gray-400 mb-6">
          {t("deleteConfirmation")}
        </p>
        <div className="flex justify-end gap-3">
          <Button variant="secondary" onClick={() => setDeleteConfirmId(null)}>
            {tCommon("buttons.cancel")}
          </Button>
          <Button
            variant="danger"
            onClick={() => deleteConfirmId && handleDelete(deleteConfirmId)}
            isLoading={deleteProduct.isPending}
          >
            {tCommon("buttons.delete")}
          </Button>
        </div>
      </Modal>
    </div>
  );
}
