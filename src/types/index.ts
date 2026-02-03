// Client types
export interface Client {
  id: string;
  name: string;
  email: string | null;
  phone: string | null;
  address: string | null;
  city: string | null;
  postal_code: string | null;
  country: string | null;
  siret: string | null;
  vat_number: string | null;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateClientInput {
  name: string;
  email?: string | null;
  phone?: string | null;
  address?: string | null;
  city?: string | null;
  postal_code?: string | null;
  country?: string | null;
  siret?: string | null;
  vat_number?: string | null;
  notes?: string | null;
}

export interface UpdateClientInput extends CreateClientInput {
  id: string;
}

// Product Category types
export interface ProductCategory {
  id: string;
  name: string;
  description: string | null;
  parent_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateProductCategoryInput {
  name: string;
  description?: string | null;
  parent_id?: string | null;
}

export interface UpdateProductCategoryInput extends CreateProductCategoryInput {
  id: string;
}

// Product types
export interface Product {
  id: string;
  designation: string;
  description: string | null;
  description_html: string | null;
  unit_price_ht: number;
  vat_rate: number;
  unit: string;
  reference: string | null;
  is_service: boolean;
  // Phase 3: Category and photo
  category_id: string | null;
  photo_path: string | null;
  // Stock management
  quantity: number | null;
  purchase_price_ht: number | null;
  created_at: string;
  updated_at: string;
}

export interface CreateProductInput {
  designation: string;
  description?: string | null;
  description_html?: string | null;
  unit_price_ht: number;
  vat_rate: number;
  unit: string;
  reference?: string | null;
  is_service: boolean;
  category_id?: string | null;
  quantity?: number | null;
  purchase_price_ht?: number | null;
}

export interface UpdateProductInput extends CreateProductInput {
  id: string;
}

// Quote types
export type QuoteStatus = "DRAFT" | "SENT" | "ACCEPTED" | "EXPIRED";

export interface Quote {
  id: string;
  quote_number: string;
  client_id: string;
  client?: Client;
  status: QuoteStatus;
  issue_date: string;
  validity_date: string;
  total_ht: number;
  total_vat: number;
  total_ttc: number;
  notes: string | null;
  notes_html: string | null;
  logo_snapshot: string | null;
  // Phase 2: Shipping costs
  shipping_cost_ht: number;
  shipping_vat_rate: number;
  // Phase 2: Down payment
  down_payment_percent: number;
  down_payment_amount: number;
  lines: QuoteLine[];
  created_at: string;
  updated_at: string;
}

export interface QuoteLine {
  id: string;
  quote_id: string;
  product_id: string | null;
  product?: Product;
  description: string;
  description_html: string | null;
  quantity: number;
  unit_price_ht: number;
  vat_rate: number;
  total_ht: number;
  total_vat: number;
  total_ttc: number;
  position: number;
  // Phase 2: Subtotals
  group_name: string | null;
  is_subtotal_line: boolean | null;
}

export interface CreateQuoteInput {
  client_id: string;
  issue_date: string;
  validity_date: string;
  notes?: string | null;
  notes_html?: string | null;
  shipping_cost_ht?: number;
  shipping_vat_rate?: number;
  down_payment_percent?: number;
  down_payment_amount?: number;
  lines: CreateQuoteLineInput[];
}

export interface CreateQuoteLineInput {
  product_id?: string | null;
  description: string;
  description_html?: string | null;
  quantity: number;
  unit_price_ht: number;
  vat_rate: number;
  group_name?: string | null;
  is_subtotal_line?: boolean;
}

export interface UpdateQuoteInput extends CreateQuoteInput {
  id: string;
  status: QuoteStatus;
}

// Invoice types
export type InvoiceStatus = "DRAFT" | "ISSUED" | "PAID";

export interface Invoice {
  id: string;
  invoice_number: string;
  client_id: string;
  client?: Client;
  quote_id: string | null;
  status: InvoiceStatus;
  issue_date: string;
  due_date: string;
  total_ht: number;
  total_vat: number;
  total_ttc: number;
  notes: string | null;
  notes_html: string | null;
  integrity_hash: string | null;
  logo_snapshot: string | null;
  // Phase 2: Shipping costs
  shipping_cost_ht: number;
  shipping_vat_rate: number;
  // Phase 2: Down payment
  down_payment_percent: number;
  down_payment_amount: number;
  is_down_payment_invoice: boolean;
  parent_quote_id: string | null;
  lines: InvoiceLine[];
  payments: Payment[];
  created_at: string;
  updated_at: string;
}

export interface InvoiceLine {
  id: string;
  invoice_id: string;
  product_id: string | null;
  product?: Product;
  description: string;
  description_html: string | null;
  quantity: number;
  unit_price_ht: number;
  vat_rate: number;
  total_ht: number;
  total_vat: number;
  total_ttc: number;
  position: number;
  // Phase 2: Subtotals
  group_name: string | null;
  is_subtotal_line: boolean | null;
}

export interface CreateInvoiceInput {
  client_id: string;
  quote_id?: string | null;
  issue_date: string;
  due_date: string;
  notes?: string | null;
  notes_html?: string | null;
  shipping_cost_ht?: number;
  shipping_vat_rate?: number;
  down_payment_percent?: number;
  down_payment_amount?: number;
  is_down_payment_invoice?: boolean;
  parent_quote_id?: string | null;
  lines: CreateInvoiceLineInput[];
}

export interface CreateInvoiceLineInput {
  product_id?: string | null;
  description: string;
  description_html?: string | null;
  quantity: number;
  unit_price_ht: number;
  vat_rate: number;
  group_name?: string | null;
  is_subtotal_line?: boolean;
}

export interface UpdateInvoiceInput {
  id: string;
  client_id: string;
  issue_date: string;
  due_date: string;
  notes?: string | null;
  notes_html?: string | null;
  shipping_cost_ht?: number;
  shipping_vat_rate?: number;
  down_payment_percent?: number;
  down_payment_amount?: number;
  lines: CreateInvoiceLineInput[];
}

// Payment types
export interface Payment {
  id: string;
  invoice_id: string;
  amount: number;
  payment_date: string;
  payment_method: string;
  reference: string | null;
  notes: string | null;
  created_at: string;
}

export interface CreatePaymentInput {
  invoice_id: string;
  amount: number;
  payment_date: string;
  payment_method: string;
  reference?: string | null;
  notes?: string | null;
}

// Company Settings types
export interface CompanySettings {
  id: string;
  company_name: string;
  address: string | null;
  city: string | null;
  postal_code: string | null;
  country: string | null;
  phone: string | null;
  email: string | null;
  website: string | null;
  siret: string | null;
  vat_number: string | null;
  logo_path: string | null;
  default_vat_rate: number;
  default_payment_terms: number;
  invoice_prefix: string;
  quote_prefix: string;
  next_invoice_number: number;
  next_quote_number: number;
  legal_mentions: string | null;
  legal_mentions_html: string | null;
  bank_details: string | null;
  // Phase 4: Delivery notes
  delivery_note_prefix: string | null;
  next_delivery_note_number: number | null;
  // Phase 8: Cloud backup
  backup_schedule: string | null;
  last_backup_date: string | null;
  cloud_provider: string | null; // Used as sync folder path
  auto_backup_enabled: boolean;
  // Phase 9: Internationalization and theming
  app_language: string | null;
  app_theme: string | null;
  // Auto-update
  auto_update_enabled: boolean;
  // Currency
  currency: string | null;
  updated_at: string;
}

export interface UpdateCompanySettingsInput {
  company_name: string;
  address?: string | null;
  city?: string | null;
  postal_code?: string | null;
  country?: string | null;
  phone?: string | null;
  email?: string | null;
  website?: string | null;
  siret?: string | null;
  vat_number?: string | null;
  default_vat_rate: number;
  default_payment_terms: number;
  invoice_prefix: string;
  quote_prefix: string;
  legal_mentions?: string | null;
  legal_mentions_html?: string | null;
  bank_details?: string | null;
  delivery_note_prefix?: string | null;
  currency?: string | null;
}

export interface UpdateAppSettingsInput {
  app_language: string;
  app_theme: string;
  auto_update_enabled: boolean;
}

export interface UpdateBackupSettingsInput {
  auto_backup_enabled: boolean;
  backup_schedule: string;
}

// Expense types
export interface Expense {
  id: string;
  name: string;
  amount: number;
  date: string;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateExpenseInput {
  name: string;
  amount: number;
  date: string;
  notes?: string | null;
}

export interface UpdateExpenseInput extends CreateExpenseInput {
  id: string;
}

// Supplier types
export interface Supplier {
  id: string;
  name: string;
  email: string | null;
  phone: string | null;
  address: string | null;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateSupplierInput {
  name: string;
  email?: string | null;
  phone?: string | null;
  address?: string | null;
  notes?: string | null;
}

export interface UpdateSupplierInput extends CreateSupplierInput {
  id: string;
}

// Product-Supplier types
export interface ProductSupplier {
  id: string;
  product_id: string;
  supplier_id: string;
  purchase_price_ht: number;
  created_at: string;
}

export interface CreateProductSupplierInput {
  product_id: string;
  supplier_id: string;
  purchase_price_ht: number;
}

export interface SupplierWithPrice {
  id: string;
  name: string;
  email: string | null;
  phone: string | null;
  purchase_price_ht: number;
  link_id: string;
}

export interface ProductWithPrice {
  id: string;
  designation: string;
  reference: string | null;
  unit_price_ht: number;
  purchase_price_ht: number;
  link_id: string;
}

export interface ProductSupplierSummary {
  product_id: string;
  supplier_id: string;
  supplier_name: string;
}

// Statistics types
export interface DashboardStats {
  total_clients: number;
  total_invoices: number;
  total_quotes: number;
  revenue_this_month: number;
  revenue_this_year: number;
  pending_payments: number;
  total_expenses: number;
  profit: number;
  recent_invoices: Invoice[];
  recent_quotes: Quote[];
}

// Delivery Note types
export type DeliveryNoteStatus = "DRAFT" | "DELIVERED" | "CANCELLED";

export interface DeliveryNote {
  id: string;
  delivery_note_number: string;
  client_id: string;
  client?: Client;
  quote_id: string | null;
  invoice_id: string | null;
  status: DeliveryNoteStatus;
  issue_date: string;
  delivery_date: string | null;
  delivery_address: string | null;
  notes: string | null;
  notes_html: string | null;
  lines: DeliveryNoteLine[];
  created_at: string;
  updated_at: string;
}

export interface DeliveryNoteLine {
  id: string;
  delivery_note_id: string;
  product_id: string | null;
  description: string;
  description_html: string | null;
  quantity: number;
  unit: string | null;
  position: number;
  created_at: string;
}

export interface CreateDeliveryNoteInput {
  client_id: string;
  quote_id?: string | null;
  invoice_id?: string | null;
  issue_date: string;
  delivery_date?: string | null;
  delivery_address?: string | null;
  notes?: string | null;
  notes_html?: string | null;
  lines: CreateDeliveryNoteLineInput[];
}

export interface CreateDeliveryNoteLineInput {
  product_id?: string | null;
  description: string;
  description_html?: string | null;
  quantity: number;
  unit?: string | null;
}

export interface UpdateDeliveryNoteInput extends CreateDeliveryNoteInput {
  id: string;
  status: DeliveryNoteStatus;
}

// Client Contact types
export interface ClientContact {
  id: string;
  client_id: string;
  name: string;
  role: string | null;
  email: string | null;
  phone: string | null;
  is_primary: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateClientContactInput {
  client_id: string;
  name: string;
  role?: string | null;
  email?: string | null;
  phone?: string | null;
  is_primary: boolean;
}

export interface UpdateClientContactInput {
  id: string;
  name: string;
  role?: string | null;
  email?: string | null;
  phone?: string | null;
  is_primary: boolean;
}

// Reminder types
export type ReminderType = "PAYMENT_DUE" | "QUOTE_EXPIRING" | "DELIVERY_SCHEDULED" | "CUSTOM";
export type DocumentType = "INVOICE" | "QUOTE" | "DELIVERY_NOTE";

export interface Reminder {
  id: string;
  reminder_type: ReminderType;
  document_type: DocumentType;
  document_id: string;
  scheduled_date: string;
  sent_at: string | null;
  message: string | null;
  created_at: string;
}

export interface CreateReminderInput {
  reminder_type: ReminderType;
  document_type: DocumentType;
  document_id: string;
  scheduled_date: string;
  message?: string | null;
}

// Report types
export interface RevenueByPeriod {
  period: string;
  revenue_ht: number;
  revenue_ttc: number;
  invoice_count: number;
}

export interface RevenueByClient {
  client_id: string;
  client_name: string;
  revenue_ht: number;
  revenue_ttc: number;
  invoice_count: number;
}

export interface ProductSales {
  product_id: string;
  product_name: string;
  quantity_sold: number;
  revenue_ht: number;
  revenue_ttc: number;
}

export interface OutstandingPayment {
  invoice_id: string;
  invoice_number: string;
  client_name: string;
  issue_date: string;
  due_date: string;
  total_ttc: number;
  days_overdue: number;
}

export interface QuoteConversionStats {
  total_quotes: number;
  converted_quotes: number;
  conversion_rate: number;
  total_quoted_amount: number;
  converted_amount: number;
}

// App Settings types
export type AppLanguage = 'system' | 'fr' | 'en' | 'ar';
export type AppTheme = 'system' | 'light' | 'dark';

// Alerts
export interface Alert {
  id: string;
  alert_type: "OVERDUE_INVOICE" | "DUE_SOON" | "EXPIRING_QUOTE" | "EXPIRED_QUOTE";
  title: string;
  message: string;
  document_type: "invoice" | "quote";
  document_id: string;
  document_number: string;
  client_name: string;
  amount: number | null;
  date: string;
  days: number; // Positive = overdue, Negative = days until
  severity: "info" | "warning" | "danger";
}

export interface AlertsSummary {
  overdue_invoices: Alert[];
  due_soon_invoices: Alert[];
  expiring_quotes: Alert[];
  expired_quotes: Alert[];
  total_overdue_amount: number;
  total_count: number;
}

// Import types
export interface ImportResult {
  added: number;
  updated: number;
  skipped: number;
  errors: string[];
}

// Auth types
export type UserRole = 'admin' | 'employee';

export type PermissionKey =
  | 'dashboard'
  | 'clients'
  | 'products'
  | 'suppliers'
  | 'quotes'
  | 'invoices'
  | 'delivery_notes'
  | 'phonebook'
  | 'reports'
  | 'expenses'
  | 'settings';

export const ALL_PERMISSIONS: PermissionKey[] = [
  'dashboard',
  'clients',
  'products',
  'suppliers',
  'quotes',
  'invoices',
  'delivery_notes',
  'phonebook',
  'reports',
  'expenses',
  'settings',
];

export interface UserInfo {
  id: string;
  username: string;
  display_name: string;
  role: UserRole;
  is_active: boolean;
  permissions: string[];
  created_at: string;
  updated_at: string;
}

export interface LoginInput {
  username: string;
  password: string;
}

export interface SetupInput {
  username: string;
  display_name: string;
  password: string;
}

export interface CreateUserInput {
  username: string;
  display_name: string;
  password: string;
  role: string;
  permissions: string[];
}

export interface UpdateUserInput {
  id: string;
  username: string;
  display_name: string;
  password?: string | null;
  role: string;
  is_active: boolean;
  permissions: string[];
}

// Database config types
export interface DatabaseConfig {
  host: string;
  port: number;
  database: string;
  username: string;
  password: string;
}

export interface DatabaseConfigSafe {
  host: string;
  port: number;
  database: string;
  username: string;
}
