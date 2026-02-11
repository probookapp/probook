import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Plus } from "lucide-react";
import { usePosStore } from "./stores/usePosStore";
import { useBarcodeScanner } from "./hooks/useBarcodeScanner";
import {
  useActiveSession,
  usePosRegisters,
  useCreatePosRegister,
  useOpenSession,
  useCloseSession,
} from "./hooks/usePosSession";
import {
  useLookupProductByBarcode,
  useCreateTransaction,
} from "./hooks/usePosTransaction";
import { ProductSearch } from "./components/ProductSearch";
import { CartDisplay } from "./components/CartDisplay";
import { CartTotals } from "./components/CartTotals";
import { PaymentModal } from "./components/PaymentModal";
import { OpenSessionModal } from "./components/OpenSessionModal";
import { CloseSessionModal } from "./components/CloseSessionModal";
import { SessionControls } from "./components/SessionControls";
import { toast } from "@/stores/useToastStore";

export function POSPage() {
  const { t } = useTranslation("pos");
  const [showPaymentModal, setShowPaymentModal] = useState(false);
  const [showOpenSessionModal, setShowOpenSessionModal] = useState(false);
  const [showCloseSessionModal, setShowCloseSessionModal] = useState(false);

  const {
    currentSession,
    currentRegister,
    setSession,
    items,
    addItem,
    clearCart,
    getFinalAmount,
  } = usePosStore();

  // Queries
  const { data: registers } = usePosRegisters();
  const { data: activeSession } = useActiveSession(currentRegister?.id);

  // Mutations
  const lookupProduct = useLookupProductByBarcode();
  const createTransaction = useCreateTransaction();
  const createRegister = useCreatePosRegister();
  const openSession = useOpenSession();
  const closeSession = useCloseSession();

  // Inline register creation
  const [showCreateRegister, setShowCreateRegister] = useState(false);
  const [newRegisterName, setNewRegisterName] = useState("");
  const [newRegisterLocation, setNewRegisterLocation] = useState("");

  // Barcode scanner
  useBarcodeScanner({
    onScan: async (barcode) => {
      if (!currentSession) {
        toast.error(t("errors.noSession"));
        return;
      }

      try {
        const product = await lookupProduct.mutateAsync(barcode);
        if (product) {
          addItem(product);
          toast.success(t("productAdded", { name: product.designation }));
        } else {
          toast.error(t("errors.productNotFound", { barcode }));
        }
      } catch {
        toast.error(t("errors.lookupFailed"));
      }
    },
  });

  // Sync session state
  useEffect(() => {
    if (activeSession && currentRegister) {
      setSession(activeSession, currentRegister);
    }
  }, [activeSession, currentRegister, setSession]);

  // Auto-select first register if none selected
  useEffect(() => {
    if (registers?.length && !currentRegister) {
      const firstActive = registers.find((r) => r.is_active);
      if (firstActive) {
        setSession(null, firstActive);
      }
    }
  }, [registers, currentRegister, setSession]);

  const handleOpenSession = async (openingFloat: number) => {
    if (!currentRegister) return;
    try {
      const session = await openSession.mutateAsync({
        register_id: currentRegister.id,
        opening_float: openingFloat,
      });
      setSession(session, currentRegister);
      setShowOpenSessionModal(false);
      toast.success(t("sessionOpened"));
    } catch (err) {
      toast.error(t("errors.openSessionFailed"));
    }
  };

  const handleCloseSession = async (actualCash: number, notes?: string) => {
    if (!currentSession) return;
    try {
      await closeSession.mutateAsync({
        session_id: currentSession.id,
        actual_cash: actualCash,
        notes,
      });
      setSession(null, currentRegister);
      clearCart();
      setShowCloseSessionModal(false);
      toast.success(t("sessionClosed"));
    } catch (err) {
      toast.error(t("errors.closeSessionFailed"));
    }
  };

  const handlePaymentComplete = async (payments: Array<{ method: string; amount: number; cashGiven?: number }>) => {
    if (!currentSession || !currentRegister || items.length === 0) return;

    const { discountPercent, discountAmount, clientId } = usePosStore.getState();

    try {
      await createTransaction.mutateAsync({
        register_id: currentRegister.id,
        session_id: currentSession.id,
        client_id: clientId,
        lines: items.map((item) => ({
          product_id: item.productId,
          barcode: item.barcode,
          designation: item.designation,
          quantity: item.quantity,
          unit_price_ht: item.unitPriceHt,
          vat_rate: item.vatRate,
          discount_percent: item.discountPercent,
        })),
        payments: payments.map((p) => ({
          payment_method: p.method as "CASH" | "CARD",
          amount: p.amount,
          cash_given: p.cashGiven,
        })),
        discount_percent: discountPercent,
        discount_amount: discountAmount,
      });

      clearCart();
      setShowPaymentModal(false);
      toast.success(t("transactionComplete"));
    } catch (err) {
      toast.error(t("errors.transactionFailed"));
    }
  };

  const handleCreateRegister = async () => {
    if (!newRegisterName.trim()) return;
    try {
      const register = await createRegister.mutateAsync({
        name: newRegisterName.trim(),
        location: newRegisterLocation.trim() || undefined,
      });
      setSession(null, register);
      setShowCreateRegister(false);
      setNewRegisterName("");
      setNewRegisterLocation("");
      toast.success(t("registerCreated"));
    } catch {
      toast.error(t("errors.createRegisterFailed"));
    }
  };

  // Show register selection or session opening if needed
  if (!currentRegister) {
    const activeRegisters = registers?.filter((r) => r.is_active) ?? [];

    return (
      <div className="h-screen flex items-center justify-center bg-muted">
        <div className="text-center">
          <h1 className="text-2xl font-bold mb-4">
            {activeRegisters.length > 0 ? t("selectRegister") : t("noRegisters")}
          </h1>
          <div className="space-y-2">
            {activeRegisters.map((register) => (
              <button
                key={register.id}
                onClick={() => setSession(null, register)}
                className="block w-full px-6 py-3 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90"
              >
                {register.name}
                {register.location && (
                  <span className="text-sm opacity-75 ml-2">
                    ({register.location})
                  </span>
                )}
              </button>
            ))}
          </div>

          {showCreateRegister ? (
            <div className="mt-4 p-4 bg-background rounded-lg border text-left max-w-sm mx-auto">
              <label className="block text-sm font-medium mb-1">
                {t("registerName")}
              </label>
              <input
                type="text"
                value={newRegisterName}
                onChange={(e) => setNewRegisterName(e.target.value)}
                placeholder={t("registerNamePlaceholder")}
                className="w-full px-3 py-2 border rounded-lg bg-background mb-3"
                autoFocus
                onKeyDown={(e) => e.key === "Enter" && handleCreateRegister()}
              />
              <label className="block text-sm font-medium mb-1">
                {t("registerLocation")}
              </label>
              <input
                type="text"
                value={newRegisterLocation}
                onChange={(e) => setNewRegisterLocation(e.target.value)}
                placeholder={t("registerLocationPlaceholder")}
                className="w-full px-3 py-2 border rounded-lg bg-background mb-4"
                onKeyDown={(e) => e.key === "Enter" && handleCreateRegister()}
              />
              <div className="flex gap-2">
                <button
                  onClick={() => setShowCreateRegister(false)}
                  className="flex-1 px-4 py-2 border rounded-lg hover:bg-muted"
                >
                  {t("cancel")}
                </button>
                <button
                  onClick={handleCreateRegister}
                  disabled={!newRegisterName.trim() || createRegister.isPending}
                  className="flex-1 px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 disabled:opacity-50"
                >
                  {t("createRegister")}
                </button>
              </div>
            </div>
          ) : (
            <button
              onClick={() => setShowCreateRegister(true)}
              className="mt-4 px-6 py-3 border-2 border-dashed border-muted-foreground/30 rounded-lg hover:border-primary hover:text-primary text-muted-foreground flex items-center gap-2 mx-auto"
            >
              <Plus className="h-5 w-5" />
              {t("createRegister")}
            </button>
          )}
        </div>
      </div>
    );
  }

  if (!currentSession) {
    return (
      <>
        <div className="h-screen flex items-center justify-center bg-muted">
          <div className="text-center">
            <h1 className="text-2xl font-bold mb-2">
              {currentRegister.name}
            </h1>
            <p className="text-muted-foreground mb-6">
              {t("sessionClosed")}
            </p>
            <button
              onClick={() => setShowOpenSessionModal(true)}
              className="px-8 py-4 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 text-lg font-medium"
            >
              {t("openSession")}
            </button>
          </div>
        </div>
        <OpenSessionModal
          open={showOpenSessionModal}
          onClose={() => setShowOpenSessionModal(false)}
          onConfirm={handleOpenSession}
          isLoading={openSession.isPending}
        />
      </>
    );
  }

  return (
    <div className="h-screen flex flex-col bg-muted">
      {/* Header */}
      <header className="h-14 bg-primary text-primary-foreground flex items-center justify-between px-4 shrink-0">
        <div className="flex items-center gap-4">
          <span className="font-bold">{currentRegister.name}</span>
          <span className="text-sm opacity-75">
            {t("ticket")}: {currentSession.id.slice(0, 8)}
          </span>
        </div>
        <SessionControls
          onCloseSession={() => setShowCloseSessionModal(true)}
        />
      </header>

      {/* Main content */}
      <div className="flex-1 flex overflow-hidden">
        {/* Cart section (60%) */}
        <div className="w-3/5 flex flex-col bg-background border-r">
          <CartDisplay />
          <CartTotals />
          {/* Payment bar */}
          <div className="p-4 border-t shrink-0">
            <button
              onClick={() => setShowPaymentModal(true)}
              disabled={items.length === 0}
              className="w-full py-4 bg-green-600 hover:bg-green-700 text-white rounded-lg font-bold text-xl disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {t("pay")} - {getFinalAmount().toFixed(2)}
            </button>
          </div>
        </div>

        {/* Product section (40%) */}
        <div className="w-2/5 flex flex-col">
          <ProductSearch onProductSelect={addItem} />
        </div>
      </div>

      {/* Modals */}
      <PaymentModal
        open={showPaymentModal}
        onClose={() => setShowPaymentModal(false)}
        onConfirm={handlePaymentComplete}
        totalAmount={getFinalAmount()}
        isLoading={createTransaction.isPending}
      />

      <CloseSessionModal
        open={showCloseSessionModal}
        onClose={() => setShowCloseSessionModal(false)}
        onConfirm={handleCloseSession}
        sessionId={currentSession.id}
        isLoading={closeSession.isPending}
      />
    </div>
  );
}
