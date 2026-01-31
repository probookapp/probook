import { useEffect } from 'react';
import { authApi, dbApi } from '@/lib/tauri';
import { useAuthStore } from '@/stores/useAuthStore';

interface AuthProviderProps {
  children: React.ReactNode;
}

export function AuthProvider({ children }: AuthProviderProps) {
  const { setUser, clearUser, setNeedsSetup, setNeedsDbSetup, setLoading } = useAuthStore();

  useEffect(() => {
    const init = async () => {
      try {
        // Check if database is configured
        const dbConfigured = await dbApi.checkConfigured();
        if (!dbConfigured) {
          setNeedsDbSetup(true);
          setLoading(false);
          return;
        }

        const needsSetup = await authApi.checkSetupRequired();
        if (needsSetup) {
          setNeedsSetup(true);
          setLoading(false);
          return;
        }

        // Try to get current session
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
  }, [setUser, clearUser, setNeedsSetup, setNeedsDbSetup, setLoading]);

  return <>{children}</>;
}
