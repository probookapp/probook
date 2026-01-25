import { z } from "zod";

export const deliveryNoteLineSchema = z.object({
  product_id: z.string().nullable().optional(),
  description: z.string().min(1, "La description est requise"),
  quantity: z.coerce.number().min(0.01, "La quantité doit être positive"),
  unit: z.string().nullable().optional(),
});

export const deliveryNoteSchema = z.object({
  client_id: z.string().min(1, "Le client est requis"),
  quote_id: z.string().nullable().optional(),
  invoice_id: z.string().nullable().optional(),
  issue_date: z.string().min(1, "La date d'émission est requise"),
  delivery_date: z.string().nullable().optional(),
  delivery_address: z.string().nullable().optional(),
  notes: z.string().nullable().optional(),
  lines: z.array(deliveryNoteLineSchema).min(1, "Au moins une ligne est requise"),
});

export type DeliveryNoteFormData = z.output<typeof deliveryNoteSchema>;
export type DeliveryNoteLineFormData = z.output<typeof deliveryNoteLineSchema>;
