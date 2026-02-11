import { apiCall } from "./api-adapter";
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
  // POS types
  PosRegister,
  CreatePosRegisterInput,
  UpdatePosRegisterInput,
  PosSession,
  OpenSessionInput,
  CloseSessionInput,
  PosTransaction,
  CreatePosTransactionInput,
  PosCashMovement,
  CreateCashMovementInput,
  PosPrinterConfig,
  CreatePrinterConfigInput,
  UpdatePrinterConfigInput,
  SessionSummary,
  DailyPosReport,
  ReceiptData,
  PrinterConnectionType,
  QueuedTransaction,
} from "@/types";
import { isTauri } from "./config";

// Client commands
export const clientApi = {
  getAll: () => apiCall<Client[]>("get_clients"),
  getById: (id: string) => apiCall<Client>("get_client", { id }),
  create: (input: CreateClientInput) => apiCall<Client>("create_client", { input }),
  update: (input: UpdateClientInput) => apiCall<Client>("update_client", { input }),
  delete: (id: string) => apiCall<void>("delete_client", { id }),
  batchDelete: (ids: string[]) => apiCall<number>("batch_delete_clients", { ids }),
};

// Product commands
export const productApi = {
  getAll: () => apiCall<Product[]>("get_products"),
  getById: (id: string) => apiCall<Product>("get_product", { id }),
  create: (input: CreateProductInput) => apiCall<Product>("create_product", { input }),
  update: (input: UpdateProductInput) => apiCall<Product>("update_product", { input }),
  delete: (id: string) => apiCall<void>("delete_product", { id }),
  batchDelete: (ids: string[]) => apiCall<number>("batch_delete_products", { ids }),
  uploadPhoto: async (productId: string, filePath: string) => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<string>("upload_product_photo", { productId, filePath });
    }
    // Web: handled via file upload endpoint (not mapped through apiCall)
    throw new Error("Photo upload requires the desktop app or a dedicated upload endpoint");
  },
  getPhotoBase64: (productId: string) =>
    apiCall<string | null>("get_product_photo_base64", { productId }),
  deletePhoto: (productId: string) =>
    apiCall<void>("delete_product_photo", { productId }),
};

// Product Category commands
export const productCategoryApi = {
  getAll: () => apiCall<ProductCategory[]>("get_product_categories"),
  getById: (id: string) => apiCall<ProductCategory>("get_product_category", { id }),
  create: (input: CreateProductCategoryInput) =>
    apiCall<ProductCategory>("create_product_category", { input }),
  update: (input: UpdateProductCategoryInput) =>
    apiCall<ProductCategory>("update_product_category", { input }),
  delete: (id: string) => apiCall<void>("delete_product_category", { id }),
};

// Quote commands
export const quoteApi = {
  getAll: () => apiCall<Quote[]>("get_quotes"),
  getById: (id: string) => apiCall<Quote>("get_quote", { id }),
  create: (input: CreateQuoteInput) => apiCall<Quote>("create_quote", { input }),
  update: (input: UpdateQuoteInput) => apiCall<Quote>("update_quote", { input }),
  delete: (id: string) => apiCall<void>("delete_quote", { id }),
  batchDelete: (ids: string[]) => apiCall<number>("batch_delete_quotes", { ids }),
  convertToInvoice: (id: string) => apiCall<Invoice>("convert_quote_to_invoice", { id }),
  convertToDeliveryNote: (id: string) => apiCall<DeliveryNote>("convert_quote_to_delivery_note", { id }),
  duplicate: (id: string) => apiCall<Quote>("duplicate_quote", { id }),
};

// Invoice commands
export const invoiceApi = {
  getAll: () => apiCall<Invoice[]>("get_invoices"),
  getById: (id: string) => apiCall<Invoice>("get_invoice", { id }),
  create: (input: CreateInvoiceInput) => apiCall<Invoice>("create_invoice", { input }),
  update: (input: UpdateInvoiceInput) => apiCall<Invoice>("update_invoice", { input }),
  delete: (id: string) => apiCall<void>("delete_invoice", { id }),
  batchDelete: (ids: string[]) => apiCall<number>("batch_delete_invoices", { ids }),
  markAsPaid: (id: string) => apiCall<Invoice>("mark_invoice_paid", { id }),
  issue: (id: string) => apiCall<Invoice>("issue_invoice", { id }),
  verifyIntegrity: (id: string) => apiCall<boolean>("verify_invoice_integrity", { id }),
  duplicate: (id: string) => apiCall<Invoice>("duplicate_invoice", { id }),
  convertToDeliveryNote: (id: string) => apiCall<DeliveryNote>("convert_invoice_to_delivery_note", { id }),
  createFromDeliveryNotes: (deliveryNoteIds: string[]) =>
    apiCall<Invoice>("create_invoice_from_delivery_notes", { deliveryNoteIds }),
};

// Payment commands
export const paymentApi = {
  getByInvoice: (invoiceId: string) => apiCall<Payment[]>("get_payments_by_invoice", { invoiceId }),
  create: (input: CreatePaymentInput) => apiCall<Payment>("create_payment", { input }),
  delete: (id: string) => apiCall<void>("delete_payment", { id }),
};

// Company Settings commands
export const settingsApi = {
  get: () => apiCall<CompanySettings>("get_company_settings"),
  update: (input: UpdateCompanySettingsInput) =>
    apiCall<CompanySettings>("update_company_settings", { input }),
  updateAppSettings: (appLanguage: string, appTheme: string, autoUpdateEnabled: boolean) =>
    apiCall<CompanySettings>("update_app_settings", { appLanguage, appTheme, autoUpdateEnabled }),
  updateBackupSettings: (autoBackupEnabled: boolean, backupSchedule: string) =>
    apiCall<CompanySettings>("update_backup_settings", { autoBackupEnabled, backupSchedule }),
  uploadLogo: async (filePath: string) => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<string>("upload_logo", { filePath });
    }
    throw new Error("Logo upload requires the desktop app or a dedicated upload endpoint");
  },
  getLogoBase64: () => apiCall<string | null>("get_logo_base64"),
  deleteLogo: () => apiCall<void>("delete_logo"),
};

// Expense commands
export const expenseApi = {
  getAll: () => apiCall<Expense[]>("get_expenses"),
  getById: (id: string) => apiCall<Expense>("get_expense", { id }),
  create: (input: CreateExpenseInput) => apiCall<Expense>("create_expense", { input }),
  update: (input: UpdateExpenseInput) => apiCall<Expense>("update_expense", { input }),
  delete: (id: string) => apiCall<void>("delete_expense", { id }),
  batchDelete: (ids: string[]) => apiCall<number>("batch_delete_expenses", { ids }),
};

// Supplier commands
export const supplierApi = {
  getAll: () => apiCall<Supplier[]>("get_suppliers"),
  getById: (id: string) => apiCall<Supplier>("get_supplier", { id }),
  create: (input: CreateSupplierInput) => apiCall<Supplier>("create_supplier", { input }),
  update: (input: UpdateSupplierInput) => apiCall<Supplier>("update_supplier", { input }),
  delete: (id: string) => apiCall<void>("delete_supplier", { id }),
  batchDelete: (ids: string[]) => apiCall<number>("batch_delete_suppliers", { ids }),
};

// Product-Supplier commands
export const productSupplierApi = {
  getAllSummaries: () => apiCall<ProductSupplierSummary[]>("get_all_product_supplier_summaries"),
  getSuppliersForProduct: (productId: string) => apiCall<SupplierWithPrice[]>("get_suppliers_for_product", { productId }),
  getProductsForSupplier: (supplierId: string) => apiCall<ProductWithPrice[]>("get_products_for_supplier", { supplierId }),
  addLink: (input: CreateProductSupplierInput) => apiCall<ProductSupplier>("add_product_supplier", { input }),
  removeLink: (linkId: string) => apiCall<void>("remove_product_supplier", { linkId }),
  updatePrice: (linkId: string, purchasePriceHt: number) => apiCall<void>("update_product_supplier_price", { linkId, purchasePriceHt }),
};

// Dashboard commands
export const dashboardApi = {
  getStats: () => apiCall<DashboardStats>("get_dashboard_stats"),
};

// Backup info type
export interface BackupInfo {
  filename: string;
  path: string;
  created_at: string;
  size_bytes: number;
}

// Backup commands (Tauri-only for file operations, limited in web mode)
export const backupApi = {
  export: async (filePath: string, password: string) => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<void>("export_backup", { filePath, password });
    }
    throw new Error("Backup export requires the desktop app");
  },
  import: async (filePath: string, password: string) => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<void>("import_backup", { filePath, password });
    }
    throw new Error("Backup import requires the desktop app");
  },
  createLocalBackup: async () => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<BackupInfo>("create_local_backup");
    }
    throw new Error("Local backup requires the desktop app");
  },
  getBackupList: async () => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<BackupInfo[]>("get_backup_list");
    }
    return [] as BackupInfo[];
  },
  openBackupsFolder: async () => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<void>("open_backups_folder");
    }
  },
  deleteBackup: async (path: string) => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<void>("delete_backup", { path });
    }
    throw new Error("Backup delete requires the desktop app");
  },
};

// Delivery Note commands
export const deliveryNoteApi = {
  getAll: () => apiCall<DeliveryNote[]>("get_delivery_notes"),
  getById: (id: string) => apiCall<DeliveryNote>("get_delivery_note", { id }),
  create: (input: CreateDeliveryNoteInput) =>
    apiCall<DeliveryNote>("create_delivery_note", { input }),
  update: (input: UpdateDeliveryNoteInput) =>
    apiCall<DeliveryNote>("update_delivery_note", { input }),
  delete: (id: string) => apiCall<void>("delete_delivery_note", { id }),
  batchDelete: (ids: string[]) => apiCall<number>("batch_delete_delivery_notes", { ids }),
  duplicate: (id: string) => apiCall<DeliveryNote>("duplicate_delivery_note", { id }),
  convertToInvoice: (id: string) => apiCall<Invoice>("convert_delivery_note_to_invoice", { id }),
};

// Client Contact commands
export const clientContactApi = {
  getAll: () => apiCall<ClientContact[]>("get_client_contacts"),
  getByClientId: (clientId: string) =>
    apiCall<ClientContact[]>("get_client_contacts_by_client", { clientId }),
  getById: (id: string) => apiCall<ClientContact>("get_client_contact", { id }),
  create: (input: CreateClientContactInput) =>
    apiCall<ClientContact>("create_client_contact", { input }),
  update: (input: UpdateClientContactInput) =>
    apiCall<ClientContact>("update_client_contact", { input }),
  delete: (id: string) => apiCall<void>("delete_client_contact", { id }),
  search: (query: string) => apiCall<ClientContact[]>("search_contacts", { query }),
};

// Reminder commands
export const reminderApi = {
  getAll: () => apiCall<Reminder[]>("get_reminders"),
  getPending: () => apiCall<Reminder[]>("get_pending_reminders"),
  getByDocument: (documentType: string, documentId: string) =>
    apiCall<Reminder[]>("get_reminders_by_document", { documentType, documentId }),
  create: (input: CreateReminderInput) =>
    apiCall<Reminder>("create_reminder", { input }),
  markSent: (id: string) => apiCall<Reminder>("mark_reminder_sent", { id }),
  delete: (id: string) => apiCall<void>("delete_reminder", { id }),
  checkAndCreate: () => apiCall<Reminder[]>("check_and_create_reminders"),
};

// Report commands
export const reportApi = {
  getRevenueByMonth: (startDate?: string, endDate?: string) =>
    apiCall<RevenueByPeriod[]>("get_revenue_by_month", { startDate, endDate }),
  getRevenueByClient: (startDate?: string, endDate?: string) =>
    apiCall<RevenueByClient[]>("get_revenue_by_client", { startDate, endDate }),
  getProductSales: (startDate?: string, endDate?: string) =>
    apiCall<ProductSales[]>("get_product_sales", { startDate, endDate }),
  getOutstandingPayments: () =>
    apiCall<OutstandingPayment[]>("get_outstanding_payments"),
  getQuoteConversionStats: (startDate?: string, endDate?: string) =>
    apiCall<QuoteConversionStats>("get_quote_conversion_stats", { startDate, endDate }),
};

// Alerts commands
export const alertsApi = {
  getSummary: () => apiCall<AlertsSummary>("get_alerts_summary"),
  markQuoteExpired: (quoteId: string) => apiCall<Quote>("mark_quote_expired", { quoteId }),
};

// Import commands (Tauri-only, requires file system access)
export const importApi = {
  importClients: async (filePath: string) => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<ImportResult>("import_clients", { filePath });
    }
    throw new Error("File import requires the desktop app");
  },
  importProducts: async (filePath: string) => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<ImportResult>("import_products", { filePath });
    }
    throw new Error("File import requires the desktop app");
  },
  importSuppliers: async (filePath: string) => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<ImportResult>("import_suppliers", { filePath });
    }
    throw new Error("File import requires the desktop app");
  },
};

// Database commands (Tauri-only, managed by server in web mode)
export const dbApi = {
  checkConfigured: async () => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<boolean>("check_db_configured");
    }
    // In web mode, database is always configured by the server
    return true;
  },
  testConnection: async (config: DatabaseConfig) => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<void>("test_db_connection", { config });
    }
    throw new Error("Database configuration is managed by the server in web mode");
  },
  saveConfig: async (config: DatabaseConfig) => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<string>("save_db_config", { config });
    }
    throw new Error("Database configuration is managed by the server in web mode");
  },
  getConfig: async () => {
    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<DatabaseConfigSafe | null>("get_db_config");
    }
    return null;
  },
};

// Auth commands
export const authApi = {
  checkSetupRequired: () => apiCall<boolean>("check_setup_required"),
  setupAdmin: (input: SetupInput) => apiCall<UserInfo>("setup_admin", { input }),
  login: (input: LoginInput) => apiCall<UserInfo>("login", { input }),
  logout: () => apiCall<void>("logout"),
  getCurrentUser: () => apiCall<UserInfo | null>("get_current_user"),
  getUsers: () => apiCall<UserInfo[]>("get_users"),
  createUser: (input: CreateUserInput) => apiCall<UserInfo>("create_user_account", { input }),
  updateUser: (input: UpdateUserInput) => apiCall<UserInfo>("update_user_account", { input }),
  deleteUser: (id: string) => apiCall<void>("delete_user_account", { id }),
  changeOwnPassword: (currentPassword: string, newPassword: string) =>
    apiCall<void>("change_own_password", { currentPassword, newPassword }),
};

// POS commands
export const posApi = {
  // Registers
  getRegisters: () => apiCall<PosRegister[]>("get_pos_registers"),
  getRegister: (id: string) => apiCall<PosRegister>("get_pos_register", { id }),
  createRegister: (input: CreatePosRegisterInput) =>
    apiCall<PosRegister>("create_pos_register", { input }),
  updateRegister: (input: UpdatePosRegisterInput) =>
    apiCall<PosRegister>("update_pos_register", { input }),
  deleteRegister: (id: string) => apiCall<void>("delete_pos_register", { id }),

  // Sessions
  getActiveSession: (registerId: string) =>
    apiCall<PosSession | null>("get_active_pos_session", { registerId }),
  openSession: (input: OpenSessionInput) =>
    apiCall<PosSession>("open_pos_session", { input }),
  closeSession: (input: CloseSessionInput) =>
    apiCall<PosSession>("close_pos_session", { input }),
  getSessionSummary: (sessionId: string) =>
    apiCall<SessionSummary>("get_pos_session_summary", { sessionId }),

  // Transactions
  lookupProductByBarcode: (barcode: string) =>
    apiCall<Product | null>("lookup_product_by_barcode", { barcode }),
  createTransaction: (input: CreatePosTransactionInput) =>
    apiCall<PosTransaction>("create_pos_transaction", { input }),
  getTransaction: (id: string) =>
    apiCall<PosTransaction>("get_pos_transaction", { id }),
  cancelTransaction: (id: string, reason: string) =>
    apiCall<PosTransaction>("cancel_pos_transaction", { id, reason }),
  getSessionTransactions: (sessionId: string) =>
    apiCall<PosTransaction[]>("get_pos_session_transactions", { sessionId }),

  // Cash movements
  createCashMovement: (input: CreateCashMovementInput) =>
    apiCall<PosCashMovement>("create_pos_cash_movement", { input }),
  getSessionCashMovements: (sessionId: string) =>
    apiCall<PosCashMovement[]>("get_pos_session_cash_movements", { sessionId }),

  // Printer configs
  getPrinterConfigs: () => apiCall<PosPrinterConfig[]>("get_pos_printer_configs"),
  createPrinterConfig: (input: CreatePrinterConfigInput) =>
    apiCall<PosPrinterConfig>("create_pos_printer_config", { input }),
  updatePrinterConfig: (input: UpdatePrinterConfigInput) =>
    apiCall<PosPrinterConfig>("update_pos_printer_config", { input }),
  deletePrinterConfig: (id: string) =>
    apiCall<void>("delete_pos_printer_config", { id }),

  // Reports
  getDailyReport: (date: string, registerId?: string) =>
    apiCall<DailyPosReport>("get_daily_pos_report", { date, registerId }),

  // Thermal Printer
  listPrinterPorts: () => apiCall<string[]>("list_printer_ports"),
  testPrinter: (connectionType: PrinterConnectionType, address: string, paperWidth: number) =>
    apiCall<void>("test_thermal_printer", { connectionType, address, paperWidth }),
  printReceipt: (
    connectionType: PrinterConnectionType,
    address: string,
    paperWidth: number,
    receipt: ReceiptData,
    currency: string,
    openDrawer: boolean
  ) =>
    apiCall<void>("print_pos_receipt", {
      connectionType,
      address,
      paperWidth,
      receipt,
      currency,
      openDrawer,
    }),

  // Offline Queue
  queueOfflineTransaction: (id: string, transactionData: string) =>
    apiCall<void>("queue_offline_transaction", { id, transactionData }),
  getPendingOfflineCount: () => apiCall<number>("get_pending_offline_count"),
  getPendingOfflineTransactions: () =>
    apiCall<QueuedTransaction[]>("get_pending_offline_transactions"),
  markOfflineTransactionSynced: (id: string) =>
    apiCall<void>("mark_offline_transaction_synced", { id }),
  markOfflineTransactionFailed: (id: string, error: string) =>
    apiCall<void>("mark_offline_transaction_failed", { id, error }),
  deleteOfflineTransaction: (id: string) =>
    apiCall<void>("delete_offline_transaction", { id }),
  checkDatabaseConnection: () => apiCall<boolean>("check_database_connection"),
};
