import { useMemo } from "react";
import { useForm, Controller, type Resolver } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button, Input, Textarea, Select, SearchableSelect } from "@/components/ui";
import { createProductSchema, type ProductFormData } from "../schemas/productSchema";
import { useProductCategories } from "../hooks/useProductCategories";
import { ProductPhotoUpload } from "./ProductPhotoUpload";
import type { Product } from "@/types";

interface ProductFormProps {
  product?: Product;
  onSubmit: (data: ProductFormData) => void;
  onCancel: () => void;
  isLoading?: boolean;
}

const vatRateOptions = [
  { value: "0", label: "0%" },
  { value: "5.5", label: "5.5%" },
  { value: "10", label: "10%" },
  { value: "20", label: "20%" },
];

export function ProductForm({ product, onSubmit, onCancel, isLoading }: ProductFormProps) {
  const { t } = useTranslation(["products", "common"]);
  const { data: categories } = useProductCategories();

  const productSchema = useMemo(() => createProductSchema(t), [t]);

  const unitOptions = useMemo(() => [
    { value: "unité", label: t("units.unit") },
    { value: "heure", label: t("units.hour") },
    { value: "jour", label: t("units.day") },
    { value: "mois", label: t("units.month") },
    { value: "forfait", label: t("units.flatRate") },
    { value: "kg", label: t("units.kg") },
    { value: "m", label: t("units.meter") },
    { value: "m²", label: t("units.squareMeter") },
    { value: "m³", label: t("units.cubicMeter") },
    { value: "l", label: t("units.liter") },
  ], [t]);

  const {
    register,
    handleSubmit,
    watch,
    control,
    formState: { errors },
  } = useForm<ProductFormData>({
    resolver: zodResolver(productSchema) as Resolver<ProductFormData>,
    defaultValues: {
      designation: product?.designation ?? "",
      description: product?.description ?? "",
      unit_price_ht: product?.unit_price_ht ?? 0,
      vat_rate: product?.vat_rate ?? 20,
      unit: product?.unit ?? "unité",
      reference: product?.reference ?? "",
      is_service: product?.is_service ?? false,
      category_id: product?.category_id ?? "",
      quantity: product?.quantity ?? 0,
      purchase_price_ht: product?.purchase_price_ht ?? 0,
    },
  });

  const isService = watch("is_service");

  const categoryOptions = [
    { value: "", label: t("fields.noCategory") },
    ...(categories?.map((cat) => ({ value: cat.id, label: cat.name })) || []),
  ];

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Input
          label={t("fields.designationRequired")}
          autoComplete="off"
          {...register("designation")}
          error={errors.designation?.message}
        />
        <Input
          label={t("fields.reference")}
          autoComplete="off"
          {...register("reference")}
          error={errors.reference?.message}
        />
      </div>

      <Textarea
        label={t("fields.description")}
        {...register("description")}
        error={errors.description?.message}
      />

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Input
          label={t("fields.unitPriceHtRequired")}
          type="number"
          step="0.01"
          {...register("unit_price_ht")}
          error={errors.unit_price_ht?.message}
        />
        <Input
          label={t("fields.purchasePriceHt")}
          type="number"
          step="0.01"
          {...register("purchase_price_ht")}
          error={errors.purchase_price_ht?.message}
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Select
          label={t("fields.vatRateRequired")}
          options={vatRateOptions}
          {...register("vat_rate")}
          error={errors.vat_rate?.message}
        />
        <Select
          label={t("fields.unitRequired")}
          options={unitOptions}
          {...register("unit")}
          error={errors.unit?.message}
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Controller
          name="category_id"
          control={control}
          render={({ field }) => (
            <SearchableSelect
              label={t("fields.category")}
              options={categoryOptions}
              value={field.value ?? ""}
              onChange={field.onChange}
              placeholder={t("fields.noCategory")}
              error={errors.category_id?.message}
            />
          )}
        />
        {!isService && (
          <Input
            label={t("fields.quantity")}
            type="number"
            step="1"
            min="0"
            {...register("quantity")}
            error={errors.quantity?.message}
          />
        )}
      </div>

      <div className="flex items-center gap-2">
        <input
          type="checkbox"
          id="is_service"
          {...register("is_service")}
          className="h-4 w-4 rounded border-gray-300 text-primary-600 focus:ring-primary-500"
        />
        <label htmlFor="is_service" className="text-sm text-gray-700 dark:text-gray-300">
          {t("fields.isServiceDescription")}
        </label>
      </div>

      {product && (
        <ProductPhotoUpload productId={product.id} />
      )}

      <div className="flex justify-end gap-3">
        <Button type="button" variant="secondary" onClick={onCancel}>
          {t("common:buttons.cancel")}
        </Button>
        <Button type="submit" isLoading={isLoading}>
          {product ? t("updateProduct") : t("createProduct")}
        </Button>
      </div>
    </form>
  );
}
