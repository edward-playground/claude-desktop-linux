import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface AppState {
  // Authentication state
  hasApiKey: boolean;
  firstRunCompleted: boolean;

  // Current state
  currentConversationId: string | null;
  isStreaming: boolean;

  // Connection status
  connectionStatus: 'connected' | 'disconnected' | 'connecting' | 'error';
  lastError: string | null;

  // Actions
  setHasApiKey: (value: boolean) => void;
  setFirstRunCompleted: (value: boolean) => void;
  setCurrentConversation: (id: string | null) => void;
  setStreaming: (value: boolean) => void;
  setConnectionStatus: (status: AppState['connectionStatus']) => void;
  setLastError: (error: string | null) => void;
  reset: () => void;
}

const initialState = {
  hasApiKey: false,
  firstRunCompleted: false,
  currentConversationId: null,
  isStreaming: false,
  connectionStatus: 'disconnected' as const,
  lastError: null,
};

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      ...initialState,

      setHasApiKey: (value) => set({ hasApiKey: value }),
      setFirstRunCompleted: (value) => set({ firstRunCompleted: value }),
      setCurrentConversation: (id) => set({ currentConversationId: id }),
      setStreaming: (value) => set({ isStreaming: value }),
      setConnectionStatus: (status) => set({ connectionStatus: status }),
      setLastError: (error) => set({ lastError: error }),
      reset: () => set(initialState),
    }),
    {
      name: 'claude-app-storage',
      partialize: (state) => ({
        currentConversationId: state.currentConversationId,
        firstRunCompleted: state.firstRunCompleted,
      }),
    }
  )
);
