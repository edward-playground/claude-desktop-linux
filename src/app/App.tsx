import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useAppStore } from '@/stores/app.store';
import { useUIStore } from '@/stores/ui.store';
import { useSettingsStore } from '@/stores/settings.store';
import { MainLayout } from '@/components/layout/MainLayout';
import { FirstRunWizard } from '@/components/wizard/FirstRunWizard';
import { Toaster } from '@/components/ui/toaster';
import { invoke } from '@tauri-apps/api/core';

export function App() {
  const { i18n } = useTranslation();
  const [isLoading, setIsLoading] = useState(true);
  const theme = useUIStore((state) => state.theme);
  const showWizard = useAppStore((state) => !state.firstRunCompleted);
  const setFirstRunCompleted = useAppStore((state) => state.setFirstRunCompleted);
  const loadSettings = useSettingsStore((state) => state.loadSettings);

  // Apply theme
  useEffect(() => {
    const root = window.document.documentElement;

    if (theme === 'system') {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light';
      root.classList.remove('light', 'dark');
      root.classList.add(systemTheme);
    } else {
      root.classList.remove('light', 'dark');
      root.classList.add(theme);
    }
  }, [theme]);

  // Listen for system theme changes
  useEffect(() => {
    if (theme !== 'system') return;

    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handler = (e: MediaQueryListEvent) => {
      const root = window.document.documentElement;
      root.classList.remove('light', 'dark');
      root.classList.add(e.matches ? 'dark' : 'light');
    };

    mediaQuery.addEventListener('change', handler);
    return () => mediaQuery.removeEventListener('change', handler);
  }, [theme]);

  // Initialize app
  useEffect(() => {
    async function init() {
      try {
        // Load settings from backend
        await loadSettings();

        // Check if first run is completed
        const firstRunCompleted = await invoke<string | null>('get_setting', {
          key: 'first_run_completed',
        });

        if (firstRunCompleted === 'true') {
          setFirstRunCompleted(true);
        }

        // Apply language from settings
        const settings = useSettingsStore.getState().settings;
        if (settings.language && settings.language !== i18n.language) {
          await i18n.changeLanguage(settings.language);
        }
      } catch (error) {
        console.error('Failed to initialize app:', error);
      } finally {
        setIsLoading(false);
      }
    }

    void init();
  }, [i18n, loadSettings, setFirstRunCompleted]);

  if (isLoading) {
    return (
      <div className="flex h-screen items-center justify-center bg-background">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    );
  }

  if (showWizard) {
    return (
      <>
        <FirstRunWizard />
        <Toaster />
      </>
    );
  }

  return (
    <>
      <MainLayout />
      <Toaster />
    </>
  );
}
