import { invoke } from "@tauri-apps/api/core";
import type {
  Client,
  CreateClientInput,
  UpdateClientInput,
  Product,
  CreateProductInput,
  UpdateProductInput,
  ProductCategory,
  CreateProductCategoryInput,
  UpdateProductCategoryInput,
  Quote,
  CreateQuoteInput,
  UpdateQuoteInput,
  Invoice,
  CreateInvoiceInput,
  UpdateInvoiceInput,
  Payment,
  CreatePaymentInput,
  CompanySettings,
  UpdateCompanySettingsInput,
  DashboardStats,
  Expense,
  CreateExpenseInput,
  UpdateExpenseInput,
  DeliveryNote,
  CreateDeliveryNoteInput,
  UpdateDeliveryNoteInput,
  ClientContact,
  CreateClientContactInput,
  UpdateClientContactInput,
  Reminder,
  CreateReminderInput,
  RevenueByPeriod,
  RevenueByClient,
  ProductSales,
  OutstandingPayment,
  QuoteConversionStats,
  AlertsSummary,
  Supplier,
  CreateSupplierInput,
  UpdateSupplierInput,
  SupplierWithPrice,
  ProductWithPrice,
  ProductSupplier,
  CreateProductSupplierInput,
  ProductSupplierSummary,
  ImportResult,
  UserInfo,
  LoginInput,
  SetupInput,
  CreateUserInput,
  UpdateUserInput,
  DatabaseConfig,
  DatabaseConfigSafe,
} from "@/types";

// Client commands
export const clientApi = {
  getAll: () => invoke<Client[]>("get_clients"),
  getById: (id: string) => invoke<Client>("get_client", { id }),
  create: (input: CreateClientInput) => invoke<Client>("create_client", { input }),
  update: (input: UpdateClientInput) => invoke<Client>("update_client", { input }),
  delete: (id: string) => invoke<void>("delete_client", { id }),
  batchDelete: (ids: string[]) => invoke<number>("batch_delete_clients", { ids }),
};

// Product commands
export const productApi = {
  getAll: () => invoke<Product[]>("get_products"),
  getById: (id: string) => invoke<Product>("get_product", { id }),
  create: (input: CreateProductInput) => invoke<Product>("create_product", { input }),
  update: (input: UpdateProductInput) => invoke<Product>("update_product", { input }),
  delete: (id: string) => invoke<void>("delete_product", { id }),
  batchDelete: (ids: string[]) => invoke<number>("batch_delete_products", { ids }),
  uploadPhoto: (productId: string, filePath: string) =>
    invoke<string>("upload_product_photo", { productId, filePath }),
  getPhotoBase64: (productId: string) =>
    invoke<string | null>("get_product_photo_base64", { productId }),
  deletePhoto: (productId: string) =>
    invoke<void>("delete_product_photo", { productId }),
};

// Product Category commands
export const productCategoryApi = {
  getAll: () => invoke<ProductCategory[]>("get_product_categories"),
  getById: (id: string) => invoke<ProductCategory>("get_product_category", { id }),
  create: (input: CreateProductCategoryInput) =>
    invoke<ProductCategory>("create_product_category", { input }),
  update: (input: UpdateProductCategoryInput) =>
    invoke<ProductCategory>("update_product_category", { input }),
  delete: (id: string) => invoke<void>("delete_product_category", { id }),
};

// Quote commands
export const quoteApi = {
  getAll: () => invoke<Quote[]>("get_quotes"),
  getById: (id: string) => invoke<Quote>("get_quote", { id }),
  create: (input: CreateQuoteInput) => invoke<Quote>("create_quote", { input }),
  update: (input: UpdateQuoteInput) => invoke<Quote>("update_quote", { input }),
  delete: (id: string) => invoke<void>("delete_quote", { id }),
  batchDelete: (ids: string[]) => invoke<number>("batch_delete_quotes", { ids }),
  convertToInvoice: (id: string) => invoke<Invoice>("convert_quote_to_invoice", { id }),
  convertToDeliveryNote: (id: string) => invoke<DeliveryNote>("convert_quote_to_delivery_note", { id }),
  duplicate: (id: string) => invoke<Quote>("duplicate_quote", { id }),
};

// Invoice commands
export const invoiceApi = {
  getAll: () => invoke<Invoice[]>("get_invoices"),
  getById: (id: string) => invoke<Invoice>("get_invoice", { id }),
  create: (input: CreateInvoiceInput) => invoke<Invoice>("create_invoice", { input }),
  update: (input: UpdateInvoiceInput) => invoke<Invoice>("update_invoice", { input }),
  delete: (id: string) => invoke<void>("delete_invoice", { id }),
  batchDelete: (ids: string[]) => invoke<number>("batch_delete_invoices", { ids }),
  markAsPaid: (id: string) => invoke<Invoice>("mark_invoice_paid", { id }),
  issue: (id: string) => invoke<Invoice>("issue_invoice", { id }),
  verifyIntegrity: (id: string) => invoke<boolean>("verify_invoice_integrity", { id }),
  duplicate: (id: string) => invoke<Invoice>("duplicate_invoice", { id }),
  convertToDeliveryNote: (id: string) => invoke<DeliveryNote>("convert_invoice_to_delivery_note", { id }),
  createFromDeliveryNotes: (deliveryNoteIds: string[]) =>
    invoke<Invoice>("create_invoice_from_delivery_notes", { deliveryNoteIds }),
};

// Payment commands
export const paymentApi = {
  getByInvoice: (invoiceId: string) => invoke<Payment[]>("get_payments_by_invoice", { invoiceId }),
  create: (input: CreatePaymentInput) => invoke<Payment>("create_payment", { input }),
  delete: (id: string) => invoke<void>("delete_payment", { id }),
};

// Company Settings commands
export const settingsApi = {
  get: () => invoke<CompanySettings>("get_company_settings"),
  update: (input: UpdateCompanySettingsInput) =>
    invoke<CompanySettings>("update_company_settings", { input }),
  updateAppSettings: (appLanguage: string, appTheme: string, autoUpdateEnabled: boolean) =>
    invoke<CompanySettings>("update_app_settings", { appLanguage, appTheme, autoUpdateEnabled }),
  uploadLogo: (filePath: string) => invoke<string>("upload_logo", { filePath }),
  getLogoBase64: () => invoke<string | null>("get_logo_base64"),
  deleteLogo: () => invoke<void>("delete_logo"),
};

// Expense commands
export const expenseApi = {
  getAll: () => invoke<Expense[]>("get_expenses"),
  getById: (id: string) => invoke<Expense>("get_expense", { id }),
  create: (input: CreateExpenseInput) => invoke<Expense>("create_expense", { input }),
  update: (input: UpdateExpenseInput) => invoke<Expense>("update_expense", { input }),
  delete: (id: string) => invoke<void>("delete_expense", { id }),
  batchDelete: (ids: string[]) => invoke<number>("batch_delete_expenses", { ids }),
};

// Supplier commands
export const supplierApi = {
  getAll: () => invoke<Supplier[]>("get_suppliers"),
  getById: (id: string) => invoke<Supplier>("get_supplier", { id }),
  create: (input: CreateSupplierInput) => invoke<Supplier>("create_supplier", { input }),
  update: (input: UpdateSupplierInput) => invoke<Supplier>("update_supplier", { input }),
  delete: (id: string) => invoke<void>("delete_supplier", { id }),
  batchDelete: (ids: string[]) => invoke<number>("batch_delete_suppliers", { ids }),
};

// Product-Supplier commands
export const productSupplierApi = {
  getAllSummaries: () => invoke<ProductSupplierSummary[]>("get_all_product_supplier_summaries"),
  getSuppliersForProduct: (productId: string) => invoke<SupplierWithPrice[]>("get_suppliers_for_product", { productId }),
  getProductsForSupplier: (supplierId: string) => invoke<ProductWithPrice[]>("get_products_for_supplier", { supplierId }),
  addLink: (input: CreateProductSupplierInput) => invoke<ProductSupplier>("add_product_supplier", { input }),
  removeLink: (linkId: string) => invoke<void>("remove_product_supplier", { linkId }),
  updatePrice: (linkId: string, purchasePriceHt: number) => invoke<void>("update_product_supplier_price", { linkId, purchasePriceHt }),
};

// Dashboard commands
export const dashboardApi = {
  getStats: () => invoke<DashboardStats>("get_dashboard_stats"),
};

// Backup info type
export interface BackupInfo {
  filename: string;
  path: string;
  created_at: string;
  size_bytes: number;
}

// Backup commands
export const backupApi = {
  export: (filePath: string, password: string) => invoke<void>("export_backup", { filePath, password }),
  import: (filePath: string, password: string) => invoke<void>("import_backup", { filePath, password }),
  createLocalBackup: () => invoke<BackupInfo>("create_local_backup"),
  getBackupList: () => invoke<BackupInfo[]>("get_backup_list"),
  openBackupsFolder: () => invoke<void>("open_backups_folder"),
  deleteBackup: (path: string) => invoke<void>("delete_backup", { path }),
};

// Delivery Note commands
export const deliveryNoteApi = {
  getAll: () => invoke<DeliveryNote[]>("get_delivery_notes"),
  getById: (id: string) => invoke<DeliveryNote>("get_delivery_note", { id }),
  create: (input: CreateDeliveryNoteInput) =>
    invoke<DeliveryNote>("create_delivery_note", { input }),
  update: (input: UpdateDeliveryNoteInput) =>
    invoke<DeliveryNote>("update_delivery_note", { input }),
  delete: (id: string) => invoke<void>("delete_delivery_note", { id }),
  batchDelete: (ids: string[]) => invoke<number>("batch_delete_delivery_notes", { ids }),
  duplicate: (id: string) => invoke<DeliveryNote>("duplicate_delivery_note", { id }),
  convertToInvoice: (id: string) => invoke<Invoice>("convert_delivery_note_to_invoice", { id }),
};

// Client Contact commands
export const clientContactApi = {
  getAll: () => invoke<ClientContact[]>("get_client_contacts"),
  getByClientId: (clientId: string) =>
    invoke<ClientContact[]>("get_client_contacts_by_client", { clientId }),
  getById: (id: string) => invoke<ClientContact>("get_client_contact", { id }),
  create: (input: CreateClientContactInput) =>
    invoke<ClientContact>("create_client_contact", { input }),
  update: (input: UpdateClientContactInput) =>
    invoke<ClientContact>("update_client_contact", { input }),
  delete: (id: string) => invoke<void>("delete_client_contact", { id }),
  search: (query: string) => invoke<ClientContact[]>("search_contacts", { query }),
};

// Reminder commands
export const reminderApi = {
  getAll: () => invoke<Reminder[]>("get_reminders"),
  getPending: () => invoke<Reminder[]>("get_pending_reminders"),
  getByDocument: (documentType: string, documentId: string) =>
    invoke<Reminder[]>("get_reminders_by_document", { documentType, documentId }),
  create: (input: CreateReminderInput) =>
    invoke<Reminder>("create_reminder", { input }),
  markSent: (id: string) => invoke<Reminder>("mark_reminder_sent", { id }),
  delete: (id: string) => invoke<void>("delete_reminder", { id }),
  checkAndCreate: () => invoke<Reminder[]>("check_and_create_reminders"),
};

// Report commands
export const reportApi = {
  getRevenueByMonth: (startDate?: string, endDate?: string) =>
    invoke<RevenueByPeriod[]>("get_revenue_by_month", { startDate, endDate }),
  getRevenueByClient: (startDate?: string, endDate?: string) =>
    invoke<RevenueByClient[]>("get_revenue_by_client", { startDate, endDate }),
  getProductSales: (startDate?: string, endDate?: string) =>
    invoke<ProductSales[]>("get_product_sales", { startDate, endDate }),
  getOutstandingPayments: () =>
    invoke<OutstandingPayment[]>("get_outstanding_payments"),
  getQuoteConversionStats: (startDate?: string, endDate?: string) =>
    invoke<QuoteConversionStats>("get_quote_conversion_stats", { startDate, endDate }),
};

// Alerts commands
export const alertsApi = {
  getSummary: () => invoke<AlertsSummary>("get_alerts_summary"),
  markQuoteExpired: (quoteId: string) => invoke<Quote>("mark_quote_expired", { quoteId }),
};

// Import commands
export const importApi = {
  importClients: (filePath: string) => invoke<ImportResult>("import_clients", { filePath }),
  importProducts: (filePath: string) => invoke<ImportResult>("import_products", { filePath }),
  importSuppliers: (filePath: string) => invoke<ImportResult>("import_suppliers", { filePath }),
};

// Database commands
export const dbApi = {
  checkConfigured: () => invoke<boolean>("check_db_configured"),
  testConnection: (config: DatabaseConfig) => invoke<void>("test_db_connection", { config }),
  saveConfig: (config: DatabaseConfig) => invoke<void>("save_db_config", { config }),
  getConfig: () => invoke<DatabaseConfigSafe | null>("get_db_config"),
};

// Auth commands
export const authApi = {
  checkSetupRequired: () => invoke<boolean>("check_setup_required"),
  setupAdmin: (input: SetupInput) => invoke<UserInfo>("setup_admin", { input }),
  login: (input: LoginInput) => invoke<UserInfo>("login", { input }),
  logout: () => invoke<void>("logout"),
  getCurrentUser: () => invoke<UserInfo | null>("get_current_user"),
  getUsers: () => invoke<UserInfo[]>("get_users"),
  createUser: (input: CreateUserInput) => invoke<UserInfo>("create_user_account", { input }),
  updateUser: (input: UpdateUserInput) => invoke<UserInfo>("update_user_account", { input }),
  deleteUser: (id: string) => invoke<void>("delete_user_account", { id }),
  changeOwnPassword: (currentPassword: string, newPassword: string) =>
    invoke<void>("change_own_password", { currentPassword, newPassword }),
};
