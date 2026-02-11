import { useEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { Plus, Monitor, MapPin, Store, ArrowLeft, Lock, Unlock } from "lucide-react";
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
import { CloseSessionModal } from "./components/CloseSessionModal";
import { TransactionHistoryDrawer } from "./components/TransactionHistoryDrawer";
import { CashMovementModal } from "./components/CashMovementModal";
import { SessionControls } from "./components/SessionControls";
import { useSettingsStore } from "@/stores/useSettingsStore";
import { toast } from "@/stores/useToastStore";

export function POSPage() {
  const { t } = useTranslation("pos");
  const navigate = useNavigate();
  const currency = useSettingsStore((state) => state.currency);
  const [showPaymentModal, setShowPaymentModal] = useState(false);
  const [showCloseSessionModal, setShowCloseSessionModal] = useState(false);
  const [showTransactionHistory, setShowTransactionHistory] = useState(false);
  const [showCashMovement, setShowCashMovement] = useState(false);

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

  // Inline session opening
  const [openingFloat, setOpeningFloat] = useState("0");

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

  // Auto-select first register only on initial mount (not after user navigates back)
  const hasAutoSelected = useRef(false);
  useEffect(() => {
    if (!hasAutoSelected.current && registers?.length && !currentRegister) {
      const firstActive = registers.find((r) => r.is_active);
      if (firstActive) {
        setSession(null, firstActive);
        hasAutoSelected.current = true;
      }
    }
  }, [registers, currentRegister, setSession]);

  const handleOpenSession = async () => {
    if (!currentRegister) return;
    try {
      const amount = parseFloat(openingFloat) || 0;
      const session = await openSession.mutateAsync({
        register_id: currentRegister.id,
        opening_float: amount,
      });
      setSession(session, currentRegister);
      setOpeningFloat("0");
      toast.success(t("sessionOpened"));
    } catch {
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
    } catch {
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
    } catch {
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

  // ─── Register selection screen ───
  if (!currentRegister) {
    const activeRegisters = registers?.filter((r) => r.is_active) ?? [];

    return (
      <div className="h-screen flex flex-col bg-(--color-bg-primary)">
        {/* Top bar */}
        <div className="h-14 border-b border-(--color-border-primary) flex items-center px-4 shrink-0">
          <button
            onClick={() => navigate("/")}
            className="flex items-center gap-2 text-sm text-(--color-text-secondary) hover:text-(--color-text-primary) transition-colors"
          >
            <ArrowLeft className="h-4 w-4" />
            {t("backToOffice")}
          </button>
        </div>

        <div className="flex-1 flex items-center justify-center p-8">
          <div className="w-full max-w-md">
            <div className="text-center mb-8">
              <div className="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-primary-50 dark:bg-primary-900/20 text-primary-600 mb-4">
                <Store className="h-8 w-8" />
              </div>
              <h1 className="text-2xl font-bold">
                {activeRegisters.length > 0 ? t("selectRegister") : t("noRegisters")}
              </h1>
            </div>

            <div className="space-y-2">
              {activeRegisters.map((register) => (
                <button
                  key={register.id}
                  onClick={() => setSession(null, register)}
                  className="w-full flex items-center gap-4 p-4 bg-(--color-bg-secondary) hover:bg-(--color-bg-tertiary) border border-(--color-border-primary) rounded-xl transition-colors text-left"
                >
                  <div className="flex items-center justify-center w-10 h-10 rounded-lg bg-primary-50 dark:bg-primary-900/20 text-primary-600 shrink-0">
                    <Monitor className="h-5 w-5" />
                  </div>
                  <div className="min-w-0">
                    <p className="font-semibold truncate">{register.name}</p>
                    {register.location && (
                      <p className="text-sm text-(--color-text-secondary) flex items-center gap-1">
                        <MapPin className="h-3 w-3" />
                        {register.location}
                      </p>
                    )}
                  </div>
                </button>
              ))}
            </div>

            {showCreateRegister ? (
              <div className="mt-4 p-5 bg-(--color-bg-secondary) rounded-xl border border-(--color-border-primary) space-y-3">
                <div>
                  <label className="block text-sm font-medium mb-1.5">
                    {t("registerName")}
                  </label>
                  <input
                    type="text"
                    value={newRegisterName}
                    onChange={(e) => setNewRegisterName(e.target.value)}
                    placeholder={t("registerNamePlaceholder")}
                    className="w-full px-3 py-2.5 border border-(--color-border-input) rounded-lg bg-(--color-bg-input) focus:outline-none focus:ring-2 focus:ring-primary-500"
                    autoFocus
                    onKeyDown={(e) => e.key === "Enter" && handleCreateRegister()}
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1.5">
                    {t("registerLocation")}
                  </label>
                  <input
                    type="text"
                    value={newRegisterLocation}
                    onChange={(e) => setNewRegisterLocation(e.target.value)}
                    placeholder={t("registerLocationPlaceholder")}
                    className="w-full px-3 py-2.5 border border-(--color-border-input) rounded-lg bg-(--color-bg-input) focus:outline-none focus:ring-2 focus:ring-primary-500"
                    onKeyDown={(e) => e.key === "Enter" && handleCreateRegister()}
                  />
                </div>
                <div className="flex gap-2 pt-1">
                  <button
                    onClick={() => setShowCreateRegister(false)}
                    className="flex-1 px-4 py-2.5 border border-(--color-border-primary) rounded-lg hover:bg-(--color-bg-secondary) font-medium transition-colors"
                  >
                    {t("cancel")}
                  </button>
                  <button
                    onClick={handleCreateRegister}
                    disabled={!newRegisterName.trim() || createRegister.isPending}
                    className="flex-1 px-4 py-2.5 bg-primary-600 text-white rounded-lg hover:bg-primary-700 font-medium disabled:opacity-50 transition-colors"
                  >
                    {createRegister.isPending ? t("loading") : t("createRegister")}
                  </button>
                </div>
              </div>
            ) : (
              <button
                onClick={() => setShowCreateRegister(true)}
                className="w-full mt-3 px-4 py-3 border-2 border-dashed border-(--color-border-secondary) rounded-xl hover:border-primary-600 hover:text-primary-600 text-(--color-text-secondary) flex items-center justify-center gap-2 transition-colors"
              >
                <Plus className="h-4 w-4" />
                {t("createRegister")}
              </button>
            )}
          </div>
        </div>
      </div>
    );
  }

  // ─── Session closed screen (inline opening float) ───
  if (!currentSession) {
    return (
      <div className="h-screen flex flex-col bg-(--color-bg-primary)">
        {/* Top bar */}
        <div className="h-14 border-b border-(--color-border-primary) flex items-center justify-between px-4 shrink-0">
          <button
            onClick={() => {
              hasAutoSelected.current = true; // Prevent auto-reselect
              setSession(null, null);
            }}
            className="flex items-center gap-2 text-sm text-(--color-text-secondary) hover:text-(--color-text-primary) transition-colors"
          >
            <ArrowLeft className="h-4 w-4" />
            {t("selectRegister")}
          </button>
          <button
            onClick={() => navigate("/")}
            className="text-sm text-(--color-text-secondary) hover:text-(--color-text-primary) transition-colors"
          >
            {t("backToOffice")}
          </button>
        </div>

        <div className="flex-1 flex items-center justify-center p-8">
          <div className="w-full max-w-sm text-center">
            <div className="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-orange-100 dark:bg-orange-900/30 text-orange-600 dark:text-orange-400 mb-4">
              <Lock className="h-8 w-8" />
            </div>
            <h1 className="text-2xl font-bold mb-1">{currentRegister.name}</h1>
            <p className="text-(--color-text-secondary) mb-8">{t("sessionClosed")}</p>

            {/* Inline opening float form */}
            <div className="bg-(--color-bg-secondary) rounded-xl border border-(--color-border-primary) p-5 text-left space-y-4">
              <p className="text-sm text-(--color-text-secondary)">
                {t("openingFloatDescription")}
              </p>
              <div>
                <label className="block text-sm font-medium mb-1.5">
                  {t("openingFloat")} ({currency})
                </label>
                <input
                  type="number"
                  value={openingFloat}
                  onChange={(e) => setOpeningFloat(e.target.value)}
                  className="w-full px-4 py-3 border border-(--color-border-input) rounded-lg text-2xl text-center font-bold bg-(--color-bg-input) focus:outline-none focus:ring-2 focus:ring-primary-500"
                  placeholder="0.00"
                  min="0"
                  step="0.01"
                  autoFocus
                  onKeyDown={(e) => e.key === "Enter" && handleOpenSession()}
                />
              </div>
              <button
                onClick={handleOpenSession}
                disabled={openSession.isPending}
                className="w-full flex items-center justify-center gap-2 px-6 py-3.5 bg-primary-600 text-white rounded-lg hover:bg-primary-700 font-semibold text-lg disabled:opacity-50 transition-colors"
              >
                <Unlock className="h-5 w-5" />
                {openSession.isPending ? t("loading") : t("openSession")}
              </button>
            </div>
          </div>
        </div>
      </div>
    );
  }

  // ─── Active session: main POS layout ───
  return (
    <div className="h-screen flex flex-col bg-(--color-bg-secondary)">
      {/* Header */}
      <header className="h-14 bg-primary-600 text-white flex items-center justify-between px-4 shrink-0">
        <div className="flex items-center gap-4">
          <span className="font-bold">{currentRegister.name}</span>
          <span className="text-sm opacity-75">
            {t("ticket")}: {currentSession.id.slice(0, 8)}
          </span>
        </div>
        <SessionControls
          onCloseSession={() => setShowCloseSessionModal(true)}
          onTransactionHistory={() => setShowTransactionHistory(true)}
          onCashMovement={() => setShowCashMovement(true)}
        />
      </header>

      {/* Main content */}
      <div className="flex-1 flex overflow-hidden">
        {/* Cart section (60%) */}
        <div className="w-3/5 flex flex-col bg-(--color-bg-primary) border-r border-(--color-border-primary)">
          <CartDisplay />
          <CartTotals />
          {/* Payment bar */}
          <div className="p-4 border-t border-(--color-border-primary) shrink-0">
            <button
              onClick={() => setShowPaymentModal(true)}
              disabled={items.length === 0}
              className="w-full py-4 bg-green-600 hover:bg-green-700 text-white rounded-lg font-bold text-xl disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
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

      <TransactionHistoryDrawer
        open={showTransactionHistory}
        onClose={() => setShowTransactionHistory(false)}
        sessionId={currentSession.id}
      />

      <CashMovementModal
        open={showCashMovement}
        onClose={() => setShowCashMovement(false)}
        sessionId={currentSession.id}
      />
    </div>
  );
}
