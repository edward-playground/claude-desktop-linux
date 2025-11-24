import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface Conversation {
  id: string;
  title: string | null;
  model: string;
  systemPrompt: string | null;
  createdAt: string;
  updatedAt: string;
  pinned: boolean;
  tags: string[];
  messageCount: number;
}

export interface Message {
  id: string;
  conversationId: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  createdAt: string;
  tokenCount: number | null;
  model: string | null;
  stopReason: string | null;
  isStreaming: boolean;
  error: string | null;
}

interface ConversationsState {
  conversations: Conversation[];
  currentMessages: Message[];
  isLoading: boolean;
  error: string | null;

  // Actions
  loadConversations: () => Promise<void>;
  loadMessages: (conversationId: string) => Promise<void>;
  createConversation: (title?: string, model?: string) => Promise<Conversation>;
  updateConversation: (
    id: string,
    updates: Partial<Pick<Conversation, 'title' | 'model' | 'systemPrompt' | 'pinned' | 'tags'>>
  ) => Promise<void>;
  deleteConversation: (id: string) => Promise<void>;
  addMessage: (message: Message) => void;
  updateMessage: (id: string, updates: Partial<Message>) => void;
  appendToMessage: (id: string, content: string) => void;
  clearMessages: () => void;
}

export const useConversationsStore = create<ConversationsState>((set, _get) => ({
  conversations: [],
  currentMessages: [],
  isLoading: false,
  error: null,

  loadConversations: async () => {
    set({ isLoading: true, error: null });
    try {
      const conversations = await invoke<Conversation[]>('get_conversations');
      set({ conversations, isLoading: false });
    } catch (error) {
      console.error('Failed to load conversations:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  loadMessages: async (conversationId) => {
    set({ isLoading: true, error: null });
    try {
      const messages = await invoke<Message[]>('get_messages', { conversationId });
      set({ currentMessages: messages, isLoading: false });
    } catch (error) {
      console.error('Failed to load messages:', error);
      set({ error: String(error), isLoading: false });
    }
  },

  createConversation: async (title, model) => {
    try {
      const conversation = await invoke<Conversation>('create_conversation', {
        request: { title, model },
      });
      set((state) => ({
        conversations: [conversation, ...state.conversations],
      }));
      return conversation;
    } catch (error) {
      console.error('Failed to create conversation:', error);
      set({ error: String(error) });
      throw error;
    }
  },

  updateConversation: async (id, updates) => {
    try {
      const conversation = await invoke<Conversation>('update_conversation', {
        id,
        request: updates,
      });
      set((state) => ({
        conversations: state.conversations.map((c) =>
          c.id === id ? conversation : c
        ),
      }));
    } catch (error) {
      console.error('Failed to update conversation:', error);
      set({ error: String(error) });
      throw error;
    }
  },

  deleteConversation: async (id) => {
    try {
      await invoke('delete_conversation', { id });
      set((state) => ({
        conversations: state.conversations.filter((c) => c.id !== id),
        currentMessages:
          state.currentMessages[0]?.conversationId === id ? [] : state.currentMessages,
      }));
    } catch (error) {
      console.error('Failed to delete conversation:', error);
      set({ error: String(error) });
      throw error;
    }
  },

  addMessage: (message) => {
    set((state) => ({
      currentMessages: [...state.currentMessages, message],
    }));
  },

  updateMessage: (id, updates) => {
    set((state) => ({
      currentMessages: state.currentMessages.map((m) =>
        m.id === id ? { ...m, ...updates } : m
      ),
    }));
  },

  appendToMessage: (id, content) => {
    set((state) => ({
      currentMessages: state.currentMessages.map((m) =>
        m.id === id ? { ...m, content: m.content + content } : m
      ),
    }));
  },

  clearMessages: () => {
    set({ currentMessages: [] });
  },
}));
