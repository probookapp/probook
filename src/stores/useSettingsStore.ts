import { create } from 'zustand';

export type AppLanguage = 'system' | 'fr' | 'en' | 'ar';
export type AppTheme = 'system' | 'light' | 'dark';
export type ResolvedTheme = 'light' | 'dark';
export type ResolvedLanguage = 'fr' | 'en' | 'ar';

interface SettingsState {
  // Language
  language: AppLanguage;
  resolvedLanguage: ResolvedLanguage;

  // Theme
  theme: AppTheme;
  resolvedTheme: ResolvedTheme;

  // Currency
  currency: string;

  // Loading state
  isInitialized: boolean;

  // Actions
  setLanguage: (language: AppLanguage) => void;
  setTheme: (theme: AppTheme) => void;
  setCurrency: (currency: string) => void;
  initializeFromBackend: (language: AppLanguage | null, theme: AppTheme | null, currency?: string | null) => void;
}

const getSystemLanguage = (): ResolvedLanguage => {
  const browserLang = navigator.language.split('-')[0];
  if (browserLang === 'fr' || browserLang === 'en' || browserLang === 'ar') {
    return browserLang;
  }
  return 'en'; // Default fallback
};

const getSystemTheme = (): ResolvedTheme => {
  if (typeof window !== 'undefined' && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return 'dark';
  }
  return 'light';
};

const applyThemeToDOM = (theme: AppTheme, resolvedTheme: ResolvedTheme) => {
  if (typeof document === 'undefined') return;

  const root = document.documentElement;
  root.classList.remove('light', 'dark');

  if (theme === 'system') {
    root.classList.add(resolvedTheme);
  } else {
    root.classList.add(theme);
  }
};

export const useSettingsStore = create<SettingsState>()((set) => ({
  // Default values (used until backend loads)
  language: 'en',
  resolvedLanguage: 'en',
  theme: 'light',
  resolvedTheme: 'light',
  currency: 'EUR',
  isInitialized: false,

  setLanguage: (language) => {
    const resolved = language === 'system' ? getSystemLanguage() : language;
    set({ language, resolvedLanguage: resolved });
  },

  setTheme: (theme) => {
    const resolved = theme === 'system' ? getSystemTheme() : theme;
    set({ theme, resolvedTheme: resolved });
    applyThemeToDOM(theme, resolved);
  },

  setCurrency: (currency) => {
    set({ currency });
  },

  initializeFromBackend: (language, theme, currency) => {
    const newLanguage = language || 'en';
    const newTheme = theme || 'light';
    const resolvedLanguage = newLanguage === 'system' ? getSystemLanguage() : newLanguage;
    const resolvedTheme = newTheme === 'system' ? getSystemTheme() : newTheme;

    set({
      language: newLanguage,
      theme: newTheme,
      currency: currency || 'EUR',
      resolvedLanguage,
      resolvedTheme,
      isInitialized: true,
    });
    applyThemeToDOM(newTheme, resolvedTheme);
  },
}));
