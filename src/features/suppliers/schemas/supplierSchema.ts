import { z } from "zod";

export const createSupplierSchema = (t: (key: string) => string) => z.object({
  name: z.string().min(1, t("suppliers:validation.nameRequired")),
  email: z
    .string()
    .email(t("suppliers:validation.emailInvalid"))
    .nullable()
    .optional()
    .or(z.literal("")),
  phone: z.string().nullable().optional(),
  address: z.string().nullable().optional(),
  notes: z.string().nullable().optional(),
});

export type SupplierFormData = z.output<ReturnType<typeof createSupplierSchema>>;
