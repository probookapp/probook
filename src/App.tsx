import { useEffect, useRef } from "react";
import { Routes, Route, Navigate } from "react-router-dom";
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

function App() {
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
        <Routes>
          <Route path="/" element={<ProtectedRoute permission="dashboard"><DashboardPage /></ProtectedRoute>} />
          <Route path="/clients" element={<ProtectedRoute permission="clients"><ClientsPage /></ProtectedRoute>} />
          <Route path="/products" element={<ProtectedRoute permission="products"><ProductsPage /></ProtectedRoute>} />
          <Route path="/suppliers" element={<ProtectedRoute permission="suppliers"><SuppliersPage /></ProtectedRoute>} />
          <Route path="/quotes" element={<ProtectedRoute permission="quotes"><QuotesPage /></ProtectedRoute>} />
          <Route path="/quotes/new" element={<ProtectedRoute permission="quotes"><QuoteFormPage /></ProtectedRoute>} />
          <Route path="/quotes/:id" element={<ProtectedRoute permission="quotes"><QuoteViewPage /></ProtectedRoute>} />
          <Route path="/quotes/:id/edit" element={<ProtectedRoute permission="quotes"><QuoteFormPage /></ProtectedRoute>} />
          <Route path="/invoices" element={<ProtectedRoute permission="invoices"><InvoicesPage /></ProtectedRoute>} />
          <Route path="/invoices/new" element={<ProtectedRoute permission="invoices"><InvoiceFormPage /></ProtectedRoute>} />
          <Route path="/invoices/:id" element={<ProtectedRoute permission="invoices"><InvoiceViewPage /></ProtectedRoute>} />
          <Route path="/invoices/:id/edit" element={<ProtectedRoute permission="invoices"><InvoiceFormPage /></ProtectedRoute>} />
          <Route path="/delivery-notes" element={<ProtectedRoute permission="delivery_notes"><DeliveryNotesPage /></ProtectedRoute>} />
          <Route path="/delivery-notes/new" element={<ProtectedRoute permission="delivery_notes"><DeliveryNoteFormPage /></ProtectedRoute>} />
          <Route path="/delivery-notes/:id" element={<ProtectedRoute permission="delivery_notes"><DeliveryNoteViewPage /></ProtectedRoute>} />
          <Route path="/delivery-notes/:id/edit" element={<ProtectedRoute permission="delivery_notes"><DeliveryNoteFormPage /></ProtectedRoute>} />
          <Route path="/phonebook" element={<ProtectedRoute permission="phonebook"><PhonebookPage /></ProtectedRoute>} />
          <Route path="/reports" element={<ProtectedRoute permission="reports"><ReportsPage /></ProtectedRoute>} />
          <Route path="/expenses" element={<ProtectedRoute permission="expenses"><ExpensesPage /></ProtectedRoute>} />
          <Route path="/settings" element={<ProtectedRoute permission="settings"><SettingsPage /></ProtectedRoute>} />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </Layout>
    </ErrorBoundary>
  );
}

export default App;
