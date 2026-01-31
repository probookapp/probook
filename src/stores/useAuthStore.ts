import { create } from 'zustand';
import type { UserInfo, PermissionKey } from '@/types';

interface AuthState {
  currentUser: UserInfo | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  needsSetup: boolean;
  needsDbSetup: boolean;

  setUser: (user: UserInfo) => void;
  clearUser: () => void;
  setNeedsSetup: (needs: boolean) => void;
  setNeedsDbSetup: (needs: boolean) => void;
  setLoading: (loading: boolean) => void;
  hasPermission: (key: PermissionKey) => boolean;
}

export const useAuthStore = create<AuthState>()((set, get) => ({
  currentUser: null,
  isAuthenticated: false,
  isLoading: true,
  needsSetup: false,
  needsDbSetup: false,

  setUser: (user) => {
    set({ currentUser: user, isAuthenticated: true, needsSetup: false });
  },

  clearUser: () => {
    set({ currentUser: null, isAuthenticated: false });
  },

  setNeedsSetup: (needs) => {
    set({ needsSetup: needs });
  },

  setNeedsDbSetup: (needs) => {
    set({ needsDbSetup: needs });
  },

  setLoading: (loading) => {
    set({ isLoading: loading });
  },

  hasPermission: (key) => {
    const { currentUser } = get();
    if (!currentUser) return false;
    if (currentUser.role === 'admin') return true;
    return currentUser.permissions.includes(key);
  },
}));
