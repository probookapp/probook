import { create } from 'zustand';
import type { LicenseStatusInfo, LicenseStatusType } from '@/types';

interface LicenseState {
  status: LicenseStatusType;
  daysRemaining: number | null;
  expiresAt: string | null;
  isWriteAllowed: boolean;
  customerName: string | null;
  licenseId: string | null;
  licenseType: string | null;
  isLoading: boolean;
  error: string | null;

  setLicenseInfo: (info: LicenseStatusInfo) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useLicenseStore = create<LicenseState>()((set) => ({
  status: 'loading',
  daysRemaining: null,
  expiresAt: null,
  isWriteAllowed: false,
  customerName: null,
  licenseId: null,
  licenseType: null,
  isLoading: true,
  error: null,

  setLicenseInfo: (info) => {
    set({
      status: info.status,
      daysRemaining: info.days_remaining,
      expiresAt: info.expires_at,
      isWriteAllowed: info.is_write_allowed,
      customerName: info.customer_name,
      licenseId: info.license_id,
      licenseType: info.license_type,
      isLoading: false,
      error: null,
    });
  },

  setLoading: (loading) => {
    set({ isLoading: loading });
  },

  setError: (error) => {
    set({ error });
  },
}));
