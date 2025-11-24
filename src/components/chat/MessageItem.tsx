import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { oneDark, oneLight } from 'react-syntax-highlighter/dist/esm/styles/prism';
import { Copy, Check, User, Bot, AlertCircle, Loader2 } from 'lucide-react';
import { type Message } from '@/stores/conversations.store';
import { useUIStore } from '@/stores/ui.store';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';

interface MessageItemProps {
  message: Message;
}

export function MessageItem({ message }: MessageItemProps) {
  const { t } = useTranslation();
  const theme = useUIStore((state) => state.theme);
  const isDark =
    theme === 'dark' ||
    (theme === 'system' &&
      window.matchMedia('(prefers-color-scheme: dark)').matches);

  const isUser = message.role === 'user';
  const isStreaming = message.isStreaming;
  const hasError = !!message.error;

  return (
    <div
      className={cn(
        'flex gap-3 rounded-lg p-4',
        isUser ? 'bg-muted/50' : 'bg-background',
        hasError && 'border border-destructive/50'
      )}
    >
      {/* Avatar */}
      <div
        className={cn(
          'flex h-8 w-8 shrink-0 items-center justify-center rounded-full',
          isUser ? 'bg-primary text-primary-foreground' : 'bg-secondary'
        )}
      >
        {isUser ? (
          <User className="h-4 w-4" />
        ) : (
          <Bot className="h-4 w-4" />
        )}
      </div>

      {/* Content */}
      <div className="min-w-0 flex-1">
        {/* Role label */}
        <div className="mb-1 text-xs font-medium text-muted-foreground">
          {isUser ? 'You' : 'Claude'}
        </div>

        {/* Message content */}
        {hasError ? (
          <div className="flex items-center gap-2 text-destructive">
            <AlertCircle className="h-4 w-4" />
            <span className="text-sm">{message.error}</span>
          </div>
        ) : (
          <div className="prose prose-sm dark:prose-invert max-w-none">
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              components={{
                code({ className, children, ...props }) {
                  const match = /language-(\w+)/.exec(className || '');
                  const isInline = !match && !className;

                  if (isInline) {
                    return (
                      <code className={className} {...props}>
                        {children}
                      </code>
                    );
                  }

                  return (
                    <CodeBlock
                      language={match?.[1] || 'text'}
                      code={String(children).replace(/\n$/, '')}
                      isDark={isDark}
                    />
                  );
                },
              }}
            >
              {message.content}
            </ReactMarkdown>
          </div>
        )}

        {/* Streaming indicator */}
        {isStreaming && (
          <div className="mt-2 flex items-center gap-2 text-xs text-muted-foreground">
            <Loader2 className="h-3 w-3 animate-spin" />
            <span>{t('chat.thinking')}</span>
          </div>
        )}
      </div>
    </div>
  );
}

interface CodeBlockProps {
  language: string;
  code: string;
  isDark: boolean;
}

function CodeBlock({ language, code, isDark }: CodeBlockProps) {
  const { t } = useTranslation();
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="group relative my-2 rounded-lg border">
      {/* Header */}
      <div className="flex items-center justify-between border-b bg-muted/50 px-3 py-1.5">
        <span className="text-xs text-muted-foreground">{language}</span>
        <Button
          variant="ghost"
          size="sm"
          className="h-6 px-2 text-xs"
          onClick={() => void handleCopy()}
        >
          {copied ? (
            <>
              <Check className="mr-1 h-3 w-3" />
              {t('common.copied')}
            </>
          ) : (
            <>
              <Copy className="mr-1 h-3 w-3" />
              {t('common.copy')}
            </>
          )}
        </Button>
      </div>

      {/* Code */}
      <SyntaxHighlighter
        language={language}
        style={isDark ? oneDark : oneLight}
        customStyle={{
          margin: 0,
          borderRadius: 0,
          fontSize: '0.875rem',
        }}
        showLineNumbers
        lineNumberStyle={{
          minWidth: '2.5em',
          paddingRight: '1em',
          color: isDark ? '#636d83' : '#999',
        }}
      >
        {code}
      </SyntaxHighlighter>
    </div>
  );
}
