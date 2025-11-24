import React from 'react';
import { useTranslation } from 'react-i18next';
import { Plus, Settings, MessageSquare, Pin, Trash2 } from 'lucide-react';
import { useAppStore } from '@/stores/app.store';
import { useUIStore } from '@/stores/ui.store';
import { useConversationsStore, type Conversation } from '@/stores/conversations.store';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { cn, formatDate, truncate } from '@/lib/utils';

export function Sidebar() {
  const { t } = useTranslation();
  const conversations = useConversationsStore((state) => state.conversations);
  const currentConversationId = useAppStore((state) => state.currentConversationId);
  const setCurrentConversation = useAppStore((state) => state.setCurrentConversation);
  const createConversation = useConversationsStore((state) => state.createConversation);
  const deleteConversation = useConversationsStore((state) => state.deleteConversation);
  const openSettingsDialog = useUIStore((state) => state.openSettingsDialog);
  const clearMessages = useConversationsStore((state) => state.clearMessages);

  const handleNewChat = async () => {
    const conv = await createConversation();
    setCurrentConversation(conv.id);
    clearMessages();
  };

  const handleSelectConversation = (id: string) => {
    setCurrentConversation(id);
  };

  const handleDeleteConversation = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    if (window.confirm(t('sidebar.deleteConfirm'))) {
      await deleteConversation(id);
      if (currentConversationId === id) {
        setCurrentConversation(null);
        clearMessages();
      }
    }
  };

  return (
    <div className="flex h-full flex-col">
      {/* Header */}
      <div className="flex items-center justify-between border-b border-sidebar-border p-4">
        <h2 className="text-sm font-semibold text-sidebar-foreground">
          {t('sidebar.conversations')}
        </h2>
        <Button
          variant="ghost"
          size="icon"
          onClick={() => void handleNewChat()}
          title={t('common.newChat')}
        >
          <Plus className="h-4 w-4" />
        </Button>
      </div>

      {/* Conversations list */}
      <ScrollArea className="flex-1">
        <div className="p-2">
          {conversations.length === 0 ? (
            <p className="px-2 py-4 text-center text-sm text-muted-foreground">
              {t('sidebar.noConversations')}
            </p>
          ) : (
            <div className="space-y-1">
              {conversations.map((conversation) => (
                <ConversationItem
                  key={conversation.id}
                  conversation={conversation}
                  isActive={conversation.id === currentConversationId}
                  onSelect={() => handleSelectConversation(conversation.id)}
                  onDelete={(e) => void handleDeleteConversation(e, conversation.id)}
                />
              ))}
            </div>
          )}
        </div>
      </ScrollArea>

      {/* Footer */}
      <div className="border-t border-sidebar-border p-4">
        <Button
          variant="ghost"
          className="w-full justify-start"
          onClick={openSettingsDialog}
        >
          <Settings className="mr-2 h-4 w-4" />
          {t('common.settings')}
        </Button>
      </div>
    </div>
  );
}

interface ConversationItemProps {
  conversation: Conversation;
  isActive: boolean;
  onSelect: () => void;
  onDelete: (e: React.MouseEvent) => void;
}

function ConversationItem({
  conversation,
  isActive,
  onSelect,
  onDelete,
}: ConversationItemProps) {
  const title = conversation.title || 'New Conversation';

  return (
    <button
      className={cn(
        'group flex w-full items-center gap-2 rounded-md px-3 py-2 text-left text-sm transition-colors',
        isActive
          ? 'bg-accent text-accent-foreground'
          : 'text-sidebar-foreground hover:bg-accent/50'
      )}
      onClick={onSelect}
    >
      <MessageSquare className="h-4 w-4 shrink-0" />
      <div className="flex-1 overflow-hidden">
        <div className="flex items-center gap-1">
          {conversation.pinned && <Pin className="h-3 w-3 text-muted-foreground" />}
          <span className="truncate">{truncate(title, 24)}</span>
        </div>
        <span className="text-xs text-muted-foreground">
          {formatDate(conversation.updatedAt)}
        </span>
      </div>
      <Button
        variant="ghost"
        size="icon"
        className="h-6 w-6 shrink-0 opacity-0 group-hover:opacity-100"
        onClick={onDelete}
      >
        <Trash2 className="h-3 w-3" />
      </Button>
    </button>
  );
}
