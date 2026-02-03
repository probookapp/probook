import { API_BASE_URL, isTauri } from "./config";

type HttpMethod = "GET" | "POST" | "PUT" | "DELETE";

interface EndpointDef {
  method: HttpMethod;
  path: string | ((args: Record<string, unknown>) => string);
  /** Extract body from args (default: send full args as body for POST/PUT) */
  body?: (args: Record<string, unknown>) => unknown;
  /** Extract query params from args for GET requests */
  query?: (args: Record<string, unknown>) => Record<string, string>;
}

// Maps Tauri command names to REST API endpoints
const COMMAND_MAP: Record<string, EndpointDef> = {
  // Clients
  get_clients: { method: "GET", path: "/api/clients" },
  get_client: { method: "GET", path: (a) => `/api/clients/${a.id}` },
  create_client: { method: "POST", path: "/api/clients", body: (a) => a.input },
  update_client: {
    method: "PUT",
    path: (a) => `/api/clients/${(a.input as Record<string, unknown>).id}`,
    body: (a) => a.input,
  },
  delete_client: { method: "DELETE", path: (a) => `/api/clients/${a.id}` },
  batch_delete_clients: {
    method: "POST",
    path: "/api/clients/batch-delete",
    body: (a) => a.ids,
  },

  // Products
  get_products: { method: "GET", path: "/api/products" },
  get_product: { method: "GET", path: (a) => `/api/products/${a.id}` },
  create_product: { method: "POST", path: "/api/products", body: (a) => a.input },
  update_product: {
    method: "PUT",
    path: (a) => `/api/products/${(a.input as Record<string, unknown>).id}`,
    body: (a) => a.input,
  },
  delete_product: { method: "DELETE", path: (a) => `/api/products/${a.id}` },
  batch_delete_products: {
    method: "POST",
    path: "/api/products/batch-delete",
    body: (a) => a.ids,
  },

  // Product Categories
  get_product_categories: { method: "GET", path: "/api/categories" },
  get_product_category: {
    method: "GET",
    path: (a) => `/api/categories/${a.id}`,
  },
  create_product_category: {
    method: "POST",
    path: "/api/categories",
    body: (a) => a.input,
  },
  update_product_category: {
    method: "PUT",
    path: (a) => `/api/categories/${(a.input as Record<string, unknown>).id}`,
    body: (a) => a.input,
  },
  delete_product_category: {
    method: "DELETE",
    path: (a) => `/api/categories/${a.id}`,
  },

  // Quotes
  get_quotes: { method: "GET", path: "/api/quotes" },
  get_quote: { method: "GET", path: (a) => `/api/quotes/${a.id}` },
  create_quote: { method: "POST", path: "/api/quotes", body: (a) => a.input },
  update_quote: {
    method: "PUT",
    path: (a) => `/api/quotes/${(a.input as Record<string, unknown>).id}`,
    body: (a) => a.input,
  },
  delete_quote: { method: "DELETE", path: (a) => `/api/quotes/${a.id}` },
  batch_delete_quotes: {
    method: "POST",
    path: "/api/quotes/batch-delete",
    body: (a) => a.ids,
  },
  convert_quote_to_invoice: {
    method: "POST",
    path: (a) => `/api/quotes/${a.id}/convert-to-invoice`,
  },
  convert_quote_to_delivery_note: {
    method: "POST",
    path: (a) => `/api/quotes/${a.id}/convert-to-delivery-note`,
  },
  duplicate_quote: {
    method: "POST",
    path: (a) => `/api/quotes/${a.id}/duplicate`,
  },

  // Invoices
  get_invoices: { method: "GET", path: "/api/invoices" },
  get_invoice: { method: "GET", path: (a) => `/api/invoices/${a.id}` },
  create_invoice: {
    method: "POST",
    path: "/api/invoices",
    body: (a) => a.input,
  },
  update_invoice: {
    method: "PUT",
    path: (a) => `/api/invoices/${(a.input as Record<string, unknown>).id}`,
    body: (a) => a.input,
  },
  delete_invoice: { method: "DELETE", path: (a) => `/api/invoices/${a.id}` },
  batch_delete_invoices: {
    method: "POST",
    path: "/api/invoices/batch-delete",
    body: (a) => a.ids,
  },
  mark_invoice_paid: {
    method: "POST",
    path: (a) => `/api/invoices/${a.id}/mark-paid`,
  },
  issue_invoice: {
    method: "POST",
    path: (a) => `/api/invoices/${a.id}/issue`,
  },
  verify_invoice_integrity: {
    method: "GET",
    path: (a) => `/api/invoices/${a.id}/verify-integrity`,
  },
  duplicate_invoice: {
    method: "POST",
    path: (a) => `/api/invoices/${a.id}/duplicate`,
  },
  convert_invoice_to_delivery_note: {
    method: "POST",
    path: (a) => `/api/invoices/${a.id}/convert-to-delivery-note`,
  },
  create_invoice_from_delivery_notes: {
    method: "POST",
    path: "/api/invoices/from-delivery-notes",
    body: (a) => a.deliveryNoteIds,
  },

  // Payments
  get_payments_by_invoice: {
    method: "GET",
    path: (a) => `/api/payments/by-invoice/${a.invoiceId}`,
  },
  create_payment: {
    method: "POST",
    path: "/api/payments",
    body: (a) => a.input,
  },
  delete_payment: { method: "DELETE", path: (a) => `/api/payments/${a.id}` },

  // Settings
  get_company_settings: { method: "GET", path: "/api/settings" },
  update_company_settings: {
    method: "PUT",
    path: "/api/settings",
    body: (a) => a.input,
  },
  update_app_settings: {
    method: "PUT",
    path: "/api/settings/app",
    body: (a) => ({
      app_language: a.appLanguage,
      app_theme: a.appTheme,
      auto_update_enabled: a.autoUpdateEnabled,
    }),
  },
  update_backup_settings: {
    method: "PUT",
    path: "/api/settings/backup",
    body: (a) => ({
      auto_backup_enabled: a.autoBackupEnabled,
      backup_schedule: a.backupSchedule,
    }),
  },
  get_dashboard_stats: { method: "GET", path: "/api/settings/dashboard" },

  // Expenses
  get_expenses: { method: "GET", path: "/api/expenses" },
  get_expense: { method: "GET", path: (a) => `/api/expenses/${a.id}` },
  create_expense: {
    method: "POST",
    path: "/api/expenses",
    body: (a) => a.input,
  },
  update_expense: {
    method: "PUT",
    path: (a) => `/api/expenses/${(a.input as Record<string, unknown>).id}`,
    body: (a) => a.input,
  },
  delete_expense: { method: "DELETE", path: (a) => `/api/expenses/${a.id}` },
  batch_delete_expenses: {
    method: "POST",
    path: "/api/expenses/batch-delete",
    body: (a) => a.ids,
  },

  // Suppliers
  get_suppliers: { method: "GET", path: "/api/suppliers" },
  get_supplier: { method: "GET", path: (a) => `/api/suppliers/${a.id}` },
  create_supplier: {
    method: "POST",
    path: "/api/suppliers",
    body: (a) => a.input,
  },
  update_supplier: {
    method: "PUT",
    path: (a) => `/api/suppliers/${(a.input as Record<string, unknown>).id}`,
    body: (a) => a.input,
  },
  delete_supplier: {
    method: "DELETE",
    path: (a) => `/api/suppliers/${a.id}`,
  },
  batch_delete_suppliers: {
    method: "POST",
    path: "/api/suppliers/batch-delete",
    body: (a) => a.ids,
  },

  // Product-Supplier links
  get_all_product_supplier_summaries: {
    method: "GET",
    path: "/api/product-suppliers/summaries",
  },
  get_suppliers_for_product: {
    method: "GET",
    path: (a) => `/api/product-suppliers/by-product/${a.productId}`,
  },
  get_products_for_supplier: {
    method: "GET",
    path: (a) => `/api/product-suppliers/by-supplier/${a.supplierId}`,
  },
  add_product_supplier: {
    method: "POST",
    path: "/api/product-suppliers",
    body: (a) => a.input,
  },
  remove_product_supplier: {
    method: "DELETE",
    path: (a) => `/api/product-suppliers/${a.linkId}`,
  },
  update_product_supplier_price: {
    method: "PUT",
    path: (a) => `/api/product-suppliers/${a.linkId}/price`,
    body: (a) => ({ purchase_price_ht: a.purchasePriceHt }),
  },

  // Delivery Notes
  get_delivery_notes: { method: "GET", path: "/api/delivery-notes" },
  get_delivery_note: {
    method: "GET",
    path: (a) => `/api/delivery-notes/${a.id}`,
  },
  create_delivery_note: {
    method: "POST",
    path: "/api/delivery-notes",
    body: (a) => a.input,
  },
  update_delivery_note: {
    method: "PUT",
    path: (a) =>
      `/api/delivery-notes/${(a.input as Record<string, unknown>).id}`,
    body: (a) => a.input,
  },
  delete_delivery_note: {
    method: "DELETE",
    path: (a) => `/api/delivery-notes/${a.id}`,
  },
  batch_delete_delivery_notes: {
    method: "POST",
    path: "/api/delivery-notes/batch-delete",
    body: (a) => a.ids,
  },
  duplicate_delivery_note: {
    method: "POST",
    path: (a) => `/api/delivery-notes/${a.id}/duplicate`,
  },
  convert_delivery_note_to_invoice: {
    method: "POST",
    path: (a) => `/api/delivery-notes/${a.id}/convert-to-invoice`,
  },

  // Client Contacts
  get_client_contacts: { method: "GET", path: "/api/contacts" },
  get_client_contacts_by_client: {
    method: "GET",
    path: (a) => `/api/contacts/by-client/${a.clientId}`,
  },
  get_client_contact: { method: "GET", path: (a) => `/api/contacts/${a.id}` },
  create_client_contact: {
    method: "POST",
    path: "/api/contacts",
    body: (a) => a.input,
  },
  update_client_contact: {
    method: "PUT",
    path: (a) => `/api/contacts/${(a.input as Record<string, unknown>).id}`,
    body: (a) => a.input,
  },
  delete_client_contact: {
    method: "DELETE",
    path: (a) => `/api/contacts/${a.id}`,
  },
  search_contacts: {
    method: "GET",
    path: "/api/contacts/search",
    query: (a) => ({ query: String(a.query || "") }),
  },

  // Reminders
  get_reminders: { method: "GET", path: "/api/reminders" },
  get_pending_reminders: { method: "GET", path: "/api/reminders/pending" },
  get_reminders_by_document: {
    method: "GET",
    path: (a) =>
      `/api/reminders/by-document/${a.documentType}/${a.documentId}`,
  },
  create_reminder: {
    method: "POST",
    path: "/api/reminders",
    body: (a) => a.input,
  },
  mark_reminder_sent: {
    method: "POST",
    path: (a) => `/api/reminders/${a.id}/mark-sent`,
  },
  delete_reminder: { method: "DELETE", path: (a) => `/api/reminders/${a.id}` },
  check_and_create_reminders: {
    method: "POST",
    path: "/api/reminders/check-and-create",
  },

  // Reports
  get_revenue_by_month: {
    method: "GET",
    path: "/api/reports/revenue-by-month",
    query: (a) => {
      const q: Record<string, string> = {};
      if (a.startDate) q.startDate = String(a.startDate);
      if (a.endDate) q.endDate = String(a.endDate);
      return q;
    },
  },
  get_revenue_by_client: {
    method: "GET",
    path: "/api/reports/revenue-by-client",
    query: (a) => {
      const q: Record<string, string> = {};
      if (a.startDate) q.startDate = String(a.startDate);
      if (a.endDate) q.endDate = String(a.endDate);
      return q;
    },
  },
  get_product_sales: {
    method: "GET",
    path: "/api/reports/product-sales",
    query: (a) => {
      const q: Record<string, string> = {};
      if (a.startDate) q.startDate = String(a.startDate);
      if (a.endDate) q.endDate = String(a.endDate);
      return q;
    },
  },
  get_outstanding_payments: {
    method: "GET",
    path: "/api/reports/outstanding-payments",
  },
  get_quote_conversion_stats: {
    method: "GET",
    path: "/api/reports/quote-conversion",
    query: (a) => {
      const q: Record<string, string> = {};
      if (a.startDate) q.startDate = String(a.startDate);
      if (a.endDate) q.endDate = String(a.endDate);
      return q;
    },
  },

  // Alerts
  get_alerts_summary: { method: "GET", path: "/api/alerts/summary" },
  mark_quote_expired: {
    method: "POST",
    path: (a) => `/api/alerts/mark-quote-expired/${a.quoteId}`,
  },

  // Auth (web mode)
  check_setup_required: { method: "GET", path: "/api/auth/setup-required" },
  setup_admin: {
    method: "POST",
    path: "/api/auth/setup",
    body: (a) => a.input,
  },
  login: { method: "POST", path: "/api/auth/login", body: (a) => a.input },
  logout: { method: "POST", path: "/api/auth/logout" },
  get_current_user: { method: "GET", path: "/api/auth/me" },
  get_users: { method: "GET", path: "/api/auth/users" },
  create_user_account: {
    method: "POST",
    path: "/api/auth/users",
    body: (a) => a.input,
  },
  update_user_account: {
    method: "PUT",
    path: (a) => `/api/auth/users/${(a.input as Record<string, unknown>).id}`,
    body: (a) => a.input,
  },
  delete_user_account: {
    method: "DELETE",
    path: (a) => `/api/auth/users/${a.id}`,
  },
  change_own_password: {
    method: "POST",
    path: "/api/auth/change-password",
    body: (a) => ({
      current_password: a.currentPassword,
      new_password: a.newPassword,
    }),
  },
};

/**
 * Unified API call that works in both Tauri (invoke) and web (fetch) modes.
 * In Tauri mode, delegates to `@tauri-apps/api/core invoke`.
 * In web mode, maps the command to a REST endpoint and uses fetch.
 */
export async function apiCall<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<T>(command, args);
  }

  const endpoint = COMMAND_MAP[command];
  if (!endpoint) {
    throw new Error(`Unknown command: ${command}. No REST mapping found.`);
  }

  const path =
    typeof endpoint.path === "function"
      ? endpoint.path(args || {})
      : endpoint.path;

  let url = `${API_BASE_URL}${path}`;

  // Append query parameters for GET requests
  if (endpoint.query && args) {
    const params = endpoint.query(args);
    const searchParams = new URLSearchParams(params);
    const qs = searchParams.toString();
    if (qs) url += `?${qs}`;
  }

  const fetchOpts: RequestInit = {
    method: endpoint.method,
    headers: { "Content-Type": "application/json" },
    credentials: "include", // send httpOnly JWT cookie
  };

  if (
    (endpoint.method === "POST" || endpoint.method === "PUT") &&
    args
  ) {
    const bodyData = endpoint.body ? endpoint.body(args) : args;
    if (bodyData !== undefined) {
      fetchOpts.body = JSON.stringify(bodyData);
    }
  }

  const res = await fetch(url, fetchOpts);

  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || `Request failed with status ${res.status}`);
  }

  // Handle empty responses (204 No Content, or empty body)
  const contentType = res.headers.get("content-type");
  if (
    res.status === 204 ||
    !contentType ||
    !contentType.includes("application/json")
  ) {
    return undefined as T;
  }

  return res.json() as Promise<T>;
}
