import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export type Theme = 'light' | 'dark' | 'system';

interface UIState {
  // Layout state
  sidebarOpen: boolean;
  sidebarWidth: number;

  // Dialogs
  settingsDialogOpen: boolean;
  deleteDialogOpen: boolean;
  deleteTarget: { type: 'conversation' | 'message'; id: string } | null;

  // Theme
  theme: Theme;

  // Actions
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;
  setSidebarWidth: (width: number) => void;
  openSettingsDialog: () => void;
  closeSettingsDialog: () => void;
  openDeleteDialog: (type: 'conversation' | 'message', id: string) => void;
  closeDeleteDialog: () => void;
  setTheme: (theme: Theme) => void;
}

export const useUIStore = create<UIState>()(
  persist(
    (set) => ({
      // Initial state
      sidebarOpen: true,
      sidebarWidth: 280,
      settingsDialogOpen: false,
      deleteDialogOpen: false,
      deleteTarget: null,
      theme: 'system',

      // Actions
      toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
      setSidebarOpen: (open) => set({ sidebarOpen: open }),
      setSidebarWidth: (width) => set({ sidebarWidth: width }),
      openSettingsDialog: () => set({ settingsDialogOpen: true }),
      closeSettingsDialog: () => set({ settingsDialogOpen: false }),
      openDeleteDialog: (type, id) =>
        set({ deleteDialogOpen: true, deleteTarget: { type, id } }),
      closeDeleteDialog: () => set({ deleteDialogOpen: false, deleteTarget: null }),
      setTheme: (theme) => set({ theme }),
    }),
    {
      name: 'claude-ui-storage',
      partialize: (state) => ({
        sidebarOpen: state.sidebarOpen,
        sidebarWidth: state.sidebarWidth,
        theme: state.theme,
      }),
    }
  )
);
