import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { Loader2, CheckCircle, XCircle, ExternalLink, Shield, Database } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useAppStore } from '@/stores/app.store';
import { useSettingsStore } from '@/stores/settings.store';
import { cn } from '@/lib/utils';

type WizardStep = 'welcome' | 'apiKey' | 'storage' | 'complete';

export function FirstRunWizard() {
  const { t } = useTranslation();
  const [step, setStep] = useState<WizardStep>('welcome');

  const renderStep = () => {
    switch (step) {
      case 'welcome':
        return <WelcomeStep onNext={() => setStep('apiKey')} />;
      case 'apiKey':
        return (
          <ApiKeyStep
            onNext={() => setStep('storage')}
            onBack={() => setStep('welcome')}
          />
        );
      case 'storage':
        return (
          <StorageStep
            onNext={() => setStep('complete')}
            onBack={() => setStep('apiKey')}
          />
        );
      case 'complete':
        return <CompleteStep />;
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-background p-4">
      <div className="w-full max-w-md rounded-lg border bg-card p-8 shadow-lg">
        {renderStep()}
      </div>
    </div>
  );
}

interface StepProps {
  onNext: () => void;
  onBack?: () => void;
}

function WelcomeStep({ onNext }: StepProps) {
  const { t } = useTranslation();

  return (
    <div className="text-center">
      <div className="mb-6">
        <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-primary/10">
          <svg
            viewBox="0 0 24 24"
            className="h-8 w-8 text-primary"
            fill="currentColor"
          >
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z" />
          </svg>
        </div>
        <h1 className="text-2xl font-bold">{t('wizard.welcome.title')}</h1>
        <p className="mt-1 text-sm text-muted-foreground">
          {t('wizard.welcome.subtitle')}
        </p>
      </div>

      <div className="mb-8 rounded-lg bg-amber-50 p-4 text-left text-sm text-amber-800 dark:bg-amber-900/20 dark:text-amber-200">
        <p>{t('wizard.welcome.disclaimer')}</p>
      </div>

      <Button onClick={onNext} className="w-full">
        {t('wizard.welcome.continue')}
      </Button>
    </div>
  );
}

function ApiKeyStep({ onNext, onBack }: StepProps) {
  const { t } = useTranslation();
  const [apiKey, setApiKey] = useState('');
  const [status, setStatus] = useState<'idle' | 'testing' | 'valid' | 'invalid'>('idle');
  const [error, setError] = useState('');
  const setHasApiKey = useAppStore((state) => state.setHasApiKey);

  const handleTest = async () => {
    if (!apiKey.trim()) return;

    setStatus('testing');
    setError('');

    try {
      await invoke('validate_api_key', { apiKey });
      await invoke('set_api_key', { apiKey });
      setStatus('valid');
      setHasApiKey(true);
    } catch (err) {
      setStatus('invalid');
      setError(String(err));
    }
  };

  const handleOpenConsole = () => {
    void open('https://console.anthropic.com/');
  };

  return (
    <div>
      <h2 className="mb-2 text-xl font-semibold">{t('wizard.apiKey.title')}</h2>
      <p className="mb-6 text-sm text-muted-foreground">
        {t('wizard.apiKey.description')}
      </p>

      <div className="mb-4 space-y-2">
        <Label htmlFor="apiKey">{t('wizard.apiKey.label')}</Label>
        <Input
          id="apiKey"
          type="password"
          value={apiKey}
          onChange={(e) => {
            setApiKey(e.target.value);
            setStatus('idle');
            setError('');
          }}
          placeholder={t('wizard.apiKey.placeholder')}
          disabled={status === 'testing'}
        />
        <Button
          variant="link"
          className="h-auto p-0 text-sm"
          onClick={handleOpenConsole}
        >
          <ExternalLink className="mr-1 h-3 w-3" />
          {t('wizard.apiKey.getKey')}
        </Button>
      </div>

      {/* Status message */}
      {status === 'testing' && (
        <div className="mb-4 flex items-center gap-2 text-sm text-muted-foreground">
          <Loader2 className="h-4 w-4 animate-spin" />
          {t('wizard.apiKey.testing')}
        </div>
      )}
      {status === 'valid' && (
        <div className="mb-4 flex items-center gap-2 text-sm text-green-600">
          <CheckCircle className="h-4 w-4" />
          {t('wizard.apiKey.valid')}
        </div>
      )}
      {status === 'invalid' && (
        <div className="mb-4 space-y-1">
          <div className="flex items-center gap-2 text-sm text-destructive">
            <XCircle className="h-4 w-4" />
            {t('wizard.apiKey.invalid')}
          </div>
          {error && <p className="text-xs text-muted-foreground">{error}</p>}
        </div>
      )}

      <div className="flex gap-2">
        <Button variant="outline" onClick={onBack}>
          {t('common.back')}
        </Button>
        {status === 'valid' ? (
          <Button onClick={onNext} className="flex-1">
            {t('common.next')}
          </Button>
        ) : (
          <Button
            onClick={() => void handleTest()}
            disabled={!apiKey.trim() || status === 'testing'}
            className="flex-1"
          >
            {status === 'testing' && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            Validate
          </Button>
        )}
      </div>
    </div>
  );
}

function StorageStep({ onNext, onBack }: StepProps) {
  const { t } = useTranslation();
  const [mode, setMode] = useState<'normal' | 'privacy'>('normal');
  const updateSettings = useSettingsStore((state) => state.updateSettings);

  const handleContinue = async () => {
    await updateSettings({ privacyMode: mode === 'privacy' });
    onNext();
  };

  return (
    <div>
      <h2 className="mb-2 text-xl font-semibold">{t('wizard.storage.title')}</h2>
      <p className="mb-6 text-sm text-muted-foreground">
        {t('wizard.storage.description')}
      </p>

      <div className="mb-6 space-y-3">
        <button
          className={cn(
            'flex w-full items-start gap-3 rounded-lg border p-4 text-left transition-colors',
            mode === 'normal'
              ? 'border-primary bg-primary/5'
              : 'border-border hover:border-primary/50'
          )}
          onClick={() => setMode('normal')}
        >
          <Database className="mt-0.5 h-5 w-5 shrink-0" />
          <div>
            <div className="font-medium">{t('wizard.storage.normal.title')}</div>
            <div className="text-sm text-muted-foreground">
              {t('wizard.storage.normal.description')}
            </div>
          </div>
        </button>

        <button
          className={cn(
            'flex w-full items-start gap-3 rounded-lg border p-4 text-left transition-colors',
            mode === 'privacy'
              ? 'border-primary bg-primary/5'
              : 'border-border hover:border-primary/50'
          )}
          onClick={() => setMode('privacy')}
        >
          <Shield className="mt-0.5 h-5 w-5 shrink-0" />
          <div>
            <div className="font-medium">{t('wizard.storage.privacy.title')}</div>
            <div className="text-sm text-muted-foreground">
              {t('wizard.storage.privacy.description')}
            </div>
          </div>
        </button>
      </div>

      <div className="flex gap-2">
        <Button variant="outline" onClick={onBack}>
          {t('common.back')}
        </Button>
        <Button onClick={() => void handleContinue()} className="flex-1">
          {t('common.next')}
        </Button>
      </div>
    </div>
  );
}

function CompleteStep() {
  const { t } = useTranslation();
  const setFirstRunCompleted = useAppStore((state) => state.setFirstRunCompleted);
  const setSetting = useSettingsStore((state) => state.setSetting);

  const handleFinish = async () => {
    await setSetting('firstRunCompleted', true);
    setFirstRunCompleted(true);
  };

  return (
    <div className="text-center">
      <div className="mb-6">
        <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-green-100 dark:bg-green-900/20">
          <CheckCircle className="h-8 w-8 text-green-600 dark:text-green-400" />
        </div>
        <h2 className="text-xl font-semibold">{t('wizard.complete.title')}</h2>
        <p className="mt-1 text-sm text-muted-foreground">
          {t('wizard.complete.description')}
        </p>
      </div>

      <div className="mb-8 space-y-2 rounded-lg bg-muted/50 p-4 text-left text-sm">
        <p>• {t('wizard.complete.tip1')}</p>
        <p>• {t('wizard.complete.tip2')}</p>
        <p>• {t('wizard.complete.tip3')}</p>
      </div>

      <Button onClick={() => void handleFinish()} className="w-full">
        {t('common.finish')}
      </Button>
    </div>
  );
}
