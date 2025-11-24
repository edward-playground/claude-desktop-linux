import { useEffect, useRef } from 'react';
import { useVirtualizer } from '@tanstack/react-virtual';
import { type Message } from '@/stores/conversations.store';
import { MessageItem } from './MessageItem';

interface MessageListProps {
  messages: Message[];
  isStreaming: boolean;
}

export function MessageList({ messages, isStreaming }: MessageListProps) {
  const parentRef = useRef<HTMLDivElement>(null);
  const bottomRef = useRef<HTMLDivElement>(null);

  // Auto-scroll to bottom when new messages arrive or streaming
  useEffect(() => {
    if (bottomRef.current) {
      bottomRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [messages, isStreaming]);

  // For very long conversations, use virtualization
  // For now, we'll use simple rendering for better UX
  // Virtual list can be enabled when message count exceeds threshold
  const useVirtual = messages.length > 100;

  const virtualizer = useVirtualizer({
    count: messages.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 150,
    overscan: 5,
    enabled: useVirtual,
  });

  if (useVirtual) {
    return (
      <div ref={parentRef} className="h-full overflow-auto scroll-smooth">
        <div
          className="relative mx-auto max-w-3xl"
          style={{ height: virtualizer.getTotalSize() }}
        >
          {virtualizer.getVirtualItems().map((virtualRow) => {
            const message = messages[virtualRow.index];
            if (!message) return null;
            return (
              <div
                key={message.id}
                className="absolute left-0 top-0 w-full"
                style={{
                  transform: `translateY(${virtualRow.start}px)`,
                }}
                ref={virtualizer.measureElement}
                data-index={virtualRow.index}
              >
                <MessageItem message={message} />
              </div>
            );
          })}
        </div>
      </div>
    );
  }

  return (
    <div ref={parentRef} className="h-full overflow-auto scroll-smooth">
      <div className="mx-auto max-w-3xl space-y-4 px-4 py-4">
        {messages.map((message) => (
          <MessageItem key={message.id} message={message} />
        ))}
        <div ref={bottomRef} />
      </div>
    </div>
  );
}
