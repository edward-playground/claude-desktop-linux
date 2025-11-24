import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface AppSettings {
  model: string;
  theme: string;
  language: string;
  streamingEnabled: boolean;
  proxyUrl: string | null;
  apiTimeout: number;
  maxTokens: number;
  temperature: number;
  firstRunCompleted: boolean;
  privacyMode: boolean;
  analyticsEnabled: boolean;
}

const defaultSettings: AppSettings = {
  model: 'claude-sonnet-4-20250514',
  theme: 'system',
  language: 'en',
  streamingEnabled: true,
  proxyUrl: null,
  apiTimeout: 120,
  maxTokens: 4096,
  temperature: 1.0,
  firstRunCompleted: false,
  privacyMode: false,
  analyticsEnabled: false,
};

interface SettingsState {
  settings: AppSettings;
  isLoading: boolean;
  error: string | null;

  // Actions
  loadSettings: () => Promise<void>;
  updateSettings: (settings: Partial<AppSettings>) => Promise<void>;
  setSetting: <K extends keyof AppSettings>(key: K, value: AppSettings[K]) => Promise<void>;
  resetSettings: () => Promise<void>;
}

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: defaultSettings,
  isLoading: false,
  error: null,

  loadSettings: async () => {
    set({ isLoading: true, error: null });
    try {
      const settings = await invoke<AppSettings>('get_settings');
      set({ settings: { ...defaultSettings, ...settings }, isLoading: false });
    } catch (error) {
      console.error('Failed to load settings:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  updateSettings: async (newSettings) => {
    const currentSettings = get().settings;
    const mergedSettings = { ...currentSettings, ...newSettings };

    set({ settings: mergedSettings });

    try {
      await invoke('update_settings', { settings: mergedSettings });
    } catch (error) {
      console.error('Failed to update settings:', error);
      // Revert on error
      set({ settings: currentSettings, error: String(error) });
      throw error;
    }
  },

  setSetting: async (key, value) => {
    const currentSettings = get().settings;
    const newSettings = { ...currentSettings, [key]: value };

    set({ settings: newSettings });

    try {
      await invoke('set_setting', { key, value: String(value) });
    } catch (error) {
      console.error(`Failed to set setting ${key}:`, error);
      // Revert on error
      set({ settings: currentSettings, error: String(error) });
      throw error;
    }
  },

  resetSettings: async () => {
    set({ settings: defaultSettings });
    try {
      await invoke('update_settings', { settings: defaultSettings });
    } catch (error) {
      console.error('Failed to reset settings:', error);
      set({ error: String(error) });
      throw error;
    }
  },
}));
