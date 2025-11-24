import { useEffect } from 'react';
import { useAppStore } from '@/stores/app.store';
import { useUIStore } from '@/stores/ui.store';
import { useConversationsStore } from '@/stores/conversations.store';
import { Sidebar } from '@/components/sidebar/Sidebar';
import { ChatView } from '@/components/chat/ChatView';
import { SettingsDialog } from '@/components/settings/SettingsDialog';
import { cn } from '@/lib/utils';

export function MainLayout() {
  const sidebarOpen = useUIStore((state) => state.sidebarOpen);
  const sidebarWidth = useUIStore((state) => state.sidebarWidth);
  const settingsDialogOpen = useUIStore((state) => state.settingsDialogOpen);
  const closeSettingsDialog = useUIStore((state) => state.closeSettingsDialog);
  const currentConversationId = useAppStore((state) => state.currentConversationId);
  const loadConversations = useConversationsStore((state) => state.loadConversations);
  const loadMessages = useConversationsStore((state) => state.loadMessages);

  // Load conversations on mount
  useEffect(() => {
    void loadConversations();
  }, [loadConversations]);

  // Load messages when conversation changes
  useEffect(() => {
    if (currentConversationId) {
      void loadMessages(currentConversationId);
    }
  }, [currentConversationId, loadMessages]);

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+, for settings
      if (e.ctrlKey && e.key === ',') {
        e.preventDefault();
        useUIStore.getState().openSettingsDialog();
      }

      // Ctrl+B for sidebar toggle
      if (e.ctrlKey && e.key === 'b') {
        e.preventDefault();
        useUIStore.getState().toggleSidebar();
      }

      // Ctrl+N for new conversation
      if (e.ctrlKey && e.key === 'n') {
        e.preventDefault();
        void useConversationsStore.getState().createConversation().then((conv) => {
          useAppStore.getState().setCurrentConversation(conv.id);
        });
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-background">
      {/* Sidebar */}
      <div
        className={cn(
          'h-full shrink-0 border-r border-sidebar-border bg-sidebar transition-all duration-200',
          sidebarOpen ? 'translate-x-0' : '-translate-x-full absolute'
        )}
        style={{ width: sidebarOpen ? sidebarWidth : 0 }}
      >
        <Sidebar />
      </div>

      {/* Main content */}
      <div className="flex flex-1 flex-col overflow-hidden">
        <ChatView />
      </div>

      {/* Settings dialog */}
      <SettingsDialog open={settingsDialogOpen} onOpenChange={closeSettingsDialog} />
    </div>
  );
}
