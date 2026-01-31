import { useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import { useSettingsStore, type AppLanguage, type AppTheme } from '@/stores/useSettingsStore';
import { settingsApi } from '@/lib/tauri';
import i18n from '@/i18n';

interface ThemeProviderProps {
  children: React.ReactNode;
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const { theme, language, resolvedLanguage, isInitialized, initializeFromBackend, setTheme } = useSettingsStore();

  // Fetch settings from backend
  const { data: settings } = useQuery({
    queryKey: ['company-settings'],
    queryFn: settingsApi.get,
  });

  // Initialize from backend when settings are loaded
  useEffect(() => {
    if (settings && !isInitialized) {
      initializeFromBackend(
        settings.app_language as AppLanguage | null,
        settings.app_theme as AppTheme | null,
        settings.currency
      );
    }
  }, [settings, isInitialized, initializeFromBackend]);

  // Sync i18n language when language changes
  useEffect(() => {
    const targetLang = language === 'system' ? resolvedLanguage : language;
    if (i18n.language !== targetLang) {
      i18n.changeLanguage(targetLang);
    }
  }, [language, resolvedLanguage]);

  // Listen for system theme preference changes
  useEffect(() => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handler = () => {
      if (theme === 'system') {
        // Re-apply system theme
        setTheme('system');
      }
    };

    mediaQuery.addEventListener('change', handler);
    return () => mediaQuery.removeEventListener('change', handler);
  }, [theme, setTheme]);

  return <>{children}</>;
}
