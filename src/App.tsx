import { Routes, Route } from "react-router-dom";
import { Layout } from "@/components/layout";
import { ErrorBoundary } from "@/components/ErrorBoundary";
import { DashboardPage } from "@/features/dashboard";
import { ClientsPage } from "@/features/clients";
import { ProductsPage } from "@/features/products";
import { QuotesPage, QuoteFormPage, QuoteViewPage } from "@/features/quotes";
import { InvoicesPage, InvoiceFormPage, InvoiceViewPage } from "@/features/invoices";
import { DeliveryNotesPage, DeliveryNoteFormPage, DeliveryNoteViewPage } from "@/features/delivery-notes";
import { PhonebookPage } from "@/features/phonebook";
import { ReportsPage } from "@/features/reports";
import { SettingsPage } from "@/features/settings";

function App() {
  return (
    <ErrorBoundary>
      <Layout>
        <Routes>
        <Route path="/" element={<DashboardPage />} />
        <Route path="/clients" element={<ClientsPage />} />
        <Route path="/products" element={<ProductsPage />} />
        <Route path="/quotes" element={<QuotesPage />} />
        <Route path="/quotes/new" element={<QuoteFormPage />} />
        <Route path="/quotes/:id" element={<QuoteViewPage />} />
        <Route path="/quotes/:id/edit" element={<QuoteFormPage />} />
        <Route path="/invoices" element={<InvoicesPage />} />
        <Route path="/invoices/new" element={<InvoiceFormPage />} />
        <Route path="/invoices/:id" element={<InvoiceViewPage />} />
        <Route path="/invoices/:id/edit" element={<InvoiceFormPage />} />
        <Route path="/delivery-notes" element={<DeliveryNotesPage />} />
        <Route path="/delivery-notes/new" element={<DeliveryNoteFormPage />} />
        <Route path="/delivery-notes/:id" element={<DeliveryNoteViewPage />} />
        <Route path="/delivery-notes/:id/edit" element={<DeliveryNoteFormPage />} />
        <Route path="/phonebook" element={<PhonebookPage />} />
        <Route path="/reports" element={<ReportsPage />} />
        <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </Layout>
    </ErrorBoundary>
  );
}

export default App;
