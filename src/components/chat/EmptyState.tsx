import { useTranslation } from 'react-i18next';
import { MessageSquare } from 'lucide-react';

export function EmptyState() {
  const { t } = useTranslation();

  return (
    <div className="flex h-full flex-col items-center justify-center text-center">
      <div className="mb-4 rounded-full bg-muted p-4">
        <MessageSquare className="h-8 w-8 text-muted-foreground" />
      </div>
      <h2 className="mb-2 text-xl font-semibold">{t('chat.empty.title')}</h2>
      <p className="max-w-md text-muted-foreground">
        {t('chat.empty.description')}
      </p>
    </div>
  );
}
