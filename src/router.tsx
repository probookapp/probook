import { useEffect, useRef } from "react";
import { createBrowserRouter, Navigate, Outlet } from "react-router-dom";
import { Layout } from "@/components/layout";
import { ErrorBoundary } from "@/components/ErrorBoundary";
import { ProtectedRoute } from "@/components/shared/ProtectedRoute";
import { useAuthStore } from "@/stores/useAuthStore";
import { LoginPage, SetupPage, DatabaseSetupPage } from "@/features/auth";
import { DashboardPage } from "@/features/dashboard";
import { ClientsPage } from "@/features/clients";
import { ProductsPage } from "@/features/products";
import { QuotesPage, QuoteFormPage, QuoteViewPage } from "@/features/quotes";
import { InvoicesPage, InvoiceFormPage, InvoiceViewPage } from "@/features/invoices";
import { DeliveryNotesPage, DeliveryNoteFormPage, DeliveryNoteViewPage } from "@/features/delivery-notes";
import { PhonebookPage } from "@/features/phonebook";
import { ReportsPage } from "@/features/reports";
import { ExpensesPage } from "@/features/expenses";
import { SuppliersPage } from "@/features/suppliers";
import { SettingsPage } from "@/features/settings";
import { isTauri } from "@/lib/config";

function AuthGate() {
  const { isLoading, needsDbSetup, needsSetup, isAuthenticated } = useAuthStore();
  const splashClosed = useRef(false);

  useEffect(() => {
    if (!isLoading && !splashClosed.current) {
      splashClosed.current = true;
      if (isTauri()) {
        (async () => {
          try {
            const { getCurrentWebviewWindow, WebviewWindow } = await import("@tauri-apps/api/webviewWindow");
            await getCurrentWebviewWindow().show();
            const splash = await WebviewWindow.getByLabel("splashscreen");
            await splash?.close();
          } catch {
            // Splash window may not exist in dev mode
          }
        })();
      }
    }
  }, [isLoading]);

  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-100 dark:bg-gray-900">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600" />
      </div>
    );
  }

  if (needsDbSetup) {
    return <DatabaseSetupPage />;
  }

  if (needsSetup) {
    return <SetupPage />;
  }

  if (!isAuthenticated) {
    return <LoginPage />;
  }

  return (
    <ErrorBoundary>
      <Layout>
        <Outlet />
      </Layout>
    </ErrorBoundary>
  );
}

export const router = createBrowserRouter([
  {
    path: "/",
    element: <AuthGate />,
    children: [
      { index: true, element: <ProtectedRoute permission="dashboard"><DashboardPage /></ProtectedRoute> },
      { path: "clients", element: <ProtectedRoute permission="clients"><ClientsPage /></ProtectedRoute> },
      { path: "products", element: <ProtectedRoute permission="products"><ProductsPage /></ProtectedRoute> },
      { path: "suppliers", element: <ProtectedRoute permission="suppliers"><SuppliersPage /></ProtectedRoute> },
      { path: "quotes", element: <ProtectedRoute permission="quotes"><QuotesPage /></ProtectedRoute> },
      { path: "quotes/new", element: <ProtectedRoute permission="quotes"><QuoteFormPage /></ProtectedRoute> },
      { path: "quotes/:id", element: <ProtectedRoute permission="quotes"><QuoteViewPage /></ProtectedRoute> },
      { path: "quotes/:id/edit", element: <ProtectedRoute permission="quotes"><QuoteFormPage /></ProtectedRoute> },
      { path: "invoices", element: <ProtectedRoute permission="invoices"><InvoicesPage /></ProtectedRoute> },
      { path: "invoices/new", element: <ProtectedRoute permission="invoices"><InvoiceFormPage /></ProtectedRoute> },
      { path: "invoices/:id", element: <ProtectedRoute permission="invoices"><InvoiceViewPage /></ProtectedRoute> },
      { path: "invoices/:id/edit", element: <ProtectedRoute permission="invoices"><InvoiceFormPage /></ProtectedRoute> },
      { path: "delivery-notes", element: <ProtectedRoute permission="delivery_notes"><DeliveryNotesPage /></ProtectedRoute> },
      { path: "delivery-notes/new", element: <ProtectedRoute permission="delivery_notes"><DeliveryNoteFormPage /></ProtectedRoute> },
      { path: "delivery-notes/:id", element: <ProtectedRoute permission="delivery_notes"><DeliveryNoteViewPage /></ProtectedRoute> },
      { path: "delivery-notes/:id/edit", element: <ProtectedRoute permission="delivery_notes"><DeliveryNoteFormPage /></ProtectedRoute> },
      { path: "phonebook", element: <ProtectedRoute permission="phonebook"><PhonebookPage /></ProtectedRoute> },
      { path: "reports", element: <ProtectedRoute permission="reports"><ReportsPage /></ProtectedRoute> },
      { path: "expenses", element: <ProtectedRoute permission="expenses"><ExpensesPage /></ProtectedRoute> },
      { path: "settings", element: <ProtectedRoute permission="settings"><SettingsPage /></ProtectedRoute> },
      { path: "*", element: <Navigate to="/" replace /> },
    ],
  },
]);
