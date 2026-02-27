import { useEffect } from 'react';
import { authApi, dbApi, licenseApi } from '@/lib/tauri';
import { useAuthStore } from '@/stores/useAuthStore';
import { useLicenseStore } from '@/stores/useLicenseStore';

interface AuthProviderProps {
  children: React.ReactNode;
}

function classifyLicenseError(err: unknown): string {
  const message = err instanceof Error ? err.message : typeof err === 'string' ? err : '';
  const lower = message.toLowerCase();
  if (lower.includes('network') || lower.includes('fetch') || lower.includes('connect') || lower.includes('timeout')) {
    return 'network';
  }
  if (lower.includes('engine') || lower.includes('lock') || lower.includes('initialize')) {
    return 'engine';
  }
  if (lower.includes('decrypt') || lower.includes('corrupt') || lower.includes('invalid') || lower.includes('parse') || lower.includes('failed to read')) {
    return 'file';
  }
  return 'unknown';
}

export function AuthProvider({ children }: AuthProviderProps) {
  const { setUser, clearUser, setNeedsSetup, setNeedsDbSetup, setLoading } = useAuthStore();
  const { setLicenseInfo, setLoading: setLicenseLoading, setError: setLicenseError } = useLicenseStore();

  useEffect(() => {
    const init = async () => {
      try {
        // Step 1: Initialize and check license (before DB — works offline)
        try {
          const licenseInfo = await licenseApi.initialize();
          setLicenseInfo(licenseInfo);
        } catch (initErr) {
          console.error("License initialize failed:", initErr);
          // If license engine fails, check status as fallback
          try {
            const licenseInfo = await licenseApi.checkStatus();
            setLicenseInfo(licenseInfo);
          } catch (statusErr) {
            console.error("License status check failed:", statusErr);
            const errorType = classifyLicenseError(initErr);
            setLicenseInfo({
              status: "no_license" as const,
              days_remaining: null,
              expires_at: null,
              is_write_allowed: false,
              customer_name: null,
              license_id: null,
              license_type: null,
            });
            setLicenseError(errorType);
          }
        }

        // Step 2: Check if database is configured
        const dbConfigured = await dbApi.checkConfigured();
        if (!dbConfigured) {
          setNeedsDbSetup(true);
          setLoading(false);
          return;
        }

        // Step 3: Check if admin setup is needed
        const needsSetup = await authApi.checkSetupRequired();
        if (needsSetup) {
          setNeedsSetup(true);
          setLoading(false);
          return;
        }

        // Step 4: Try to get current session
        const user = await authApi.getCurrentUser();
        if (user) {
          setUser(user);
        } else {
          clearUser();
        }
      } catch {
        clearUser();
      } finally {
        setLoading(false);
      }
    };

    init();
  }, [setUser, clearUser, setNeedsSetup, setNeedsDbSetup, setLoading, setLicenseInfo, setLicenseLoading, setLicenseError]);

  return <>{children}</>;
}
