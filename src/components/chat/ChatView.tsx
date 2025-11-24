import { useCallback, useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { Menu, Send, Square, Loader2 } from 'lucide-react';
import { useAppStore } from '@/stores/app.store';
import { useUIStore } from '@/stores/ui.store';
import { useSettingsStore } from '@/stores/settings.store';
import { useConversationsStore, type Message } from '@/stores/conversations.store';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { MessageList } from './MessageList';
import { ModelSelector } from './ModelSelector';
import { EmptyState } from './EmptyState';
import { useToast } from '@/hooks/useToast';
import { generateId } from '@/lib/utils';

interface StreamChunk {
  chunkType: string;
  delta?: string;
  stopReason?: string;
  error?: string;
}

export function ChatView() {
  const { t } = useTranslation();
  const { toast } = useToast();

  const [input, setInput] = useState('');
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const currentConversationId = useAppStore((state) => state.currentConversationId);
  const isStreaming = useAppStore((state) => state.isStreaming);
  const setStreaming = useAppStore((state) => state.setStreaming);
  const toggleSidebar = useUIStore((state) => state.toggleSidebar);
  const settings = useSettingsStore((state) => state.settings);
  const messages = useConversationsStore((state) => state.currentMessages);
  const addMessage = useConversationsStore((state) => state.addMessage);
  const updateMessage = useConversationsStore((state) => state.updateMessage);
  const appendToMessage = useConversationsStore((state) => state.appendToMessage);
  const createConversation = useConversationsStore((state) => state.createConversation);
  const setCurrentConversation = useAppStore((state) => state.setCurrentConversation);

  // Auto-resize textarea
  useEffect(() => {
    const textarea = textareaRef.current;
    if (textarea) {
      textarea.style.height = 'auto';
      textarea.style.height = `${Math.min(textarea.scrollHeight, 200)}px`;
    }
  }, [input]);

  const handleSend = useCallback(async () => {
    const trimmedInput = input.trim();
    if (!trimmedInput || isStreaming) return;

    let conversationId = currentConversationId;

    // Create new conversation if needed
    if (!conversationId) {
      try {
        const conv = await createConversation(trimmedInput.slice(0, 50));
        conversationId = conv.id;
        setCurrentConversation(conversationId);
      } catch (error) {
        toast({
          variant: 'destructive',
          title: t('common.error'),
          description: String(error),
        });
        return;
      }
    }

    // Add user message
    const userMessage: Message = {
      id: generateId(),
      conversationId,
      role: 'user',
      content: trimmedInput,
      createdAt: new Date().toISOString(),
      tokenCount: null,
      model: null,
      stopReason: null,
      isStreaming: false,
      error: null,
    };

    addMessage(userMessage);

    // Save user message to database
    try {
      await invoke('create_message', {
        request: {
          conversationId,
          role: 'user',
          content: trimmedInput,
          model: null,
        },
      });
    } catch (error) {
      console.error('Failed to save user message:', error);
    }

    setInput('');
    setStreaming(true);

    // Create placeholder for assistant response
    const assistantMessageId = generateId();
    const assistantMessage: Message = {
      id: assistantMessageId,
      conversationId,
      role: 'assistant',
      content: '',
      createdAt: new Date().toISOString(),
      tokenCount: null,
      model: settings.model,
      stopReason: null,
      isStreaming: true,
      error: null,
    };

    addMessage(assistantMessage);

    try {
      // Listen for streaming events
      const eventName = `stream-${conversationId}`;
      const unlisten = await listen<StreamChunk>(eventName, (event) => {
        const chunk = event.payload;

        if (chunk.chunkType === 'content_block_delta' && chunk.delta) {
          appendToMessage(assistantMessageId, chunk.delta);
        } else if (chunk.chunkType === 'message_delta' && chunk.stopReason) {
          updateMessage(assistantMessageId, {
            stopReason: chunk.stopReason,
            isStreaming: false,
          });
        } else if (chunk.chunkType === 'message_stop') {
          updateMessage(assistantMessageId, { isStreaming: false });
          setStreaming(false);
        } else if (chunk.chunkType === 'error' && chunk.error) {
          updateMessage(assistantMessageId, {
            error: chunk.error,
            isStreaming: false,
          });
          setStreaming(false);
          toast({
            variant: 'destructive',
            title: t('common.error'),
            description: chunk.error,
          });
        }
      });

      // Send message to API
      const allMessages = [...messages, userMessage];
      await invoke('send_message', {
        conversationId,
        messages: allMessages,
        model: settings.model,
        systemPrompt: null,
        stream: settings.streamingEnabled,
      });

      // Clean up listener after a timeout (in case message_stop is missed)
      setTimeout(() => {
        unlisten();
        setStreaming(false);
      }, 120000); // 2 minutes timeout

    } catch (error) {
      console.error('Failed to send message:', error);
      updateMessage(assistantMessageId, {
        error: String(error),
        isStreaming: false,
      });
      setStreaming(false);

      toast({
        variant: 'destructive',
        title: t('common.error'),
        description: String(error),
      });
    }
  }, [
    input,
    isStreaming,
    currentConversationId,
    messages,
    settings.model,
    settings.streamingEnabled,
    addMessage,
    appendToMessage,
    updateMessage,
    createConversation,
    setCurrentConversation,
    setStreaming,
    toast,
    t,
  ]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      void handleSend();
    }
  };

  const handleStop = () => {
    setStreaming(false);
    // The streaming will naturally stop when we stop listening
  };

  return (
    <div className="flex h-full flex-col">
      {/* Header */}
      <header className="flex h-14 items-center justify-between border-b px-4">
        <div className="flex items-center gap-2">
          <Button variant="ghost" size="icon" onClick={toggleSidebar}>
            <Menu className="h-5 w-5" />
          </Button>
          <ModelSelector />
        </div>
      </header>

      {/* Messages area */}
      <div className="flex-1 overflow-hidden">
        {messages.length === 0 ? (
          <EmptyState />
        ) : (
          <MessageList messages={messages} isStreaming={isStreaming} />
        )}
      </div>

      {/* Input area */}
      <div className="border-t p-4">
        <div className="mx-auto flex max-w-3xl gap-2">
          <Textarea
            ref={textareaRef}
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder={t('chat.placeholder')}
            className="min-h-[44px] max-h-[200px] resize-none"
            disabled={isStreaming}
            rows={1}
          />
          {isStreaming ? (
            <Button variant="destructive" size="icon" onClick={handleStop}>
              <Square className="h-4 w-4" />
            </Button>
          ) : (
            <Button
              size="icon"
              onClick={() => void handleSend()}
              disabled={!input.trim()}
            >
              <Send className="h-4 w-4" />
            </Button>
          )}
        </div>
        <p className="mt-2 text-center text-xs text-muted-foreground">
          Ctrl+Enter {t('chat.send')}
        </p>
      </div>
    </div>
  );
}
