import { create } from "zustand";
import type { Product, PosSession, PosRegister } from "@/types";

export interface CartItem {
  id: string; // Temporary ID for cart management
  productId: string | null;
  barcode: string | null;
  designation: string;
  quantity: number;
  unitPriceHt: number;
  vatRate: number;
  unit: string;
  discountPercent: number;
}

interface PosState {
  // Session state
  currentSession: PosSession | null;
  currentRegister: PosRegister | null;

  // Cart state
  items: CartItem[];
  discountPercent: number;
  discountAmount: number;
  clientId: string | null;

  // Actions
  setSession: (session: PosSession | null, register: PosRegister | null) => void;
  clearSession: () => void;

  // Cart actions
  addItem: (product: Product, quantity?: number) => void;
  addCustomItem: (designation: string, unitPriceHt: number, vatRate: number, quantity?: number) => void;
  removeItem: (itemId: string) => void;
  updateQuantity: (itemId: string, quantity: number) => void;
  updateItemPrice: (itemId: string, unitPriceHt: number) => void;
  updateItemDiscount: (itemId: string, discountPercent: number) => void;
  setTransactionDiscount: (percent: number, amount: number) => void;
  setClient: (clientId: string | null) => void;
  clearCart: () => void;

  // Computed getters
  getSubtotalHt: () => number;
  getTotalVat: () => number;
  getTotalTtc: () => number;
  getFinalAmount: () => number;
  getItemCount: () => number;
}

export const usePosStore = create<PosState>((set, get) => ({
  // Initial state
  currentSession: null,
  currentRegister: null,
  items: [],
  discountPercent: 0,
  discountAmount: 0,
  clientId: null,

  // Session actions
  setSession: (session, register) =>
    set({ currentSession: session, currentRegister: register }),

  clearSession: () =>
    set({
      currentSession: null,
      currentRegister: null,
      items: [],
      discountPercent: 0,
      discountAmount: 0,
      clientId: null,
    }),

  // Cart actions
  addItem: (product, quantity = 1) => {
    const { items } = get();
    const existingItem = items.find((item) => item.productId === product.id);

    if (existingItem) {
      set({
        items: items.map((item) =>
          item.productId === product.id
            ? { ...item, quantity: item.quantity + quantity }
            : item
        ),
      });
    } else {
      const newItem: CartItem = {
        id: crypto.randomUUID(),
        productId: product.id,
        barcode: product.barcode,
        designation: product.designation,
        quantity,
        unitPriceHt: product.unit_price_ht,
        vatRate: product.vat_rate,
        unit: product.unit ?? "unit",
        discountPercent: 0,
      };
      set({ items: [...items, newItem] });
    }
  },

  addCustomItem: (designation, unitPriceHt, vatRate, quantity = 1) => {
    const newItem: CartItem = {
      id: crypto.randomUUID(),
      productId: null,
      barcode: null,
      designation,
      quantity,
      unitPriceHt,
      vatRate,
      unit: "unit",
      discountPercent: 0,
    };
    set((state) => ({ items: [...state.items, newItem] }));
  },

  removeItem: (itemId) =>
    set((state) => ({
      items: state.items.filter((item) => item.id !== itemId),
    })),

  updateQuantity: (itemId, quantity) => {
    if (quantity <= 0) {
      get().removeItem(itemId);
      return;
    }
    set((state) => ({
      items: state.items.map((item) =>
        item.id === itemId ? { ...item, quantity } : item
      ),
    }));
  },

  updateItemPrice: (itemId, unitPriceHt) =>
    set((state) => ({
      items: state.items.map((item) =>
        item.id === itemId ? { ...item, unitPriceHt } : item
      ),
    })),

  updateItemDiscount: (itemId, discountPercent) =>
    set((state) => ({
      items: state.items.map((item) =>
        item.id === itemId ? { ...item, discountPercent } : item
      ),
    })),

  setTransactionDiscount: (percent, amount) =>
    set({ discountPercent: percent, discountAmount: amount }),

  setClient: (clientId) => set({ clientId }),

  clearCart: () =>
    set({
      items: [],
      discountPercent: 0,
      discountAmount: 0,
      clientId: null,
    }),

  // Computed getters
  getSubtotalHt: () => {
    const { items } = get();
    return items.reduce((total, item) => {
      const baseHt = item.quantity * item.unitPriceHt;
      const discountedHt = baseHt * (1 - item.discountPercent / 100);
      return total + discountedHt;
    }, 0);
  },

  getTotalVat: () => {
    const { items } = get();
    return items.reduce((total, item) => {
      const baseHt = item.quantity * item.unitPriceHt;
      const discountedHt = baseHt * (1 - item.discountPercent / 100);
      const vat = discountedHt * (item.vatRate / 100);
      return total + vat;
    }, 0);
  },

  getTotalTtc: () => {
    return get().getSubtotalHt() + get().getTotalVat();
  },

  getFinalAmount: () => {
    const { discountPercent, discountAmount } = get();
    const totalTtc = get().getTotalTtc();
    return totalTtc * (1 - discountPercent / 100) - discountAmount;
  },

  getItemCount: () => {
    return get().items.reduce((count, item) => count + item.quantity, 0);
  },
}));
