import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { useUIStore, type Theme } from '@/stores/ui.store';
import { useSettingsStore } from '@/stores/settings.store';
import { useAppStore } from '@/stores/app.store';
import { useToast } from '@/hooks/useToast';

interface SettingsDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function SettingsDialog({ open, onOpenChange }: SettingsDialogProps) {
  const { t } = useTranslation();

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>{t('settings.title')}</DialogTitle>
        </DialogHeader>
        <Tabs defaultValue="general" className="mt-4">
          <TabsList className="grid w-full grid-cols-4">
            <TabsTrigger value="general">{t('settings.general.title')}</TabsTrigger>
            <TabsTrigger value="api">{t('settings.api.title')}</TabsTrigger>
            <TabsTrigger value="privacy">{t('settings.privacy.title')}</TabsTrigger>
            <TabsTrigger value="about">{t('settings.about.title')}</TabsTrigger>
          </TabsList>
          <TabsContent value="general" className="mt-4">
            <GeneralSettings />
          </TabsContent>
          <TabsContent value="api" className="mt-4">
            <ApiSettings />
          </TabsContent>
          <TabsContent value="privacy" className="mt-4">
            <PrivacySettings />
          </TabsContent>
          <TabsContent value="about" className="mt-4">
            <AboutSettings />
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  );
}

function GeneralSettings() {
  const { t, i18n } = useTranslation();
  const theme = useUIStore((state) => state.theme);
  const setTheme = useUIStore((state) => state.setTheme);
  const setSetting = useSettingsStore((state) => state.setSetting);

  const handleThemeChange = (value: Theme) => {
    setTheme(value);
    void setSetting('theme', value);
  };

  const handleLanguageChange = async (value: string) => {
    await i18n.changeLanguage(value);
    await setSetting('language', value);
  };

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <Label>{t('settings.general.theme')}</Label>
        <Select value={theme} onValueChange={handleThemeChange}>
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="light">{t('settings.general.themeLight')}</SelectItem>
            <SelectItem value="dark">{t('settings.general.themeDark')}</SelectItem>
            <SelectItem value="system">{t('settings.general.themeSystem')}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div className="space-y-2">
        <Label>{t('settings.general.language')}</Label>
        <Select value={i18n.language} onValueChange={(v) => void handleLanguageChange(v)}>
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="en">English</SelectItem>
            <SelectItem value="zh-TW">繁體中文</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>
  );
}

function ApiSettings() {
  const { t } = useTranslation();
  const { toast } = useToast();
  const settings = useSettingsStore((state) => state.settings);
  const updateSettings = useSettingsStore((state) => state.updateSettings);
  const hasApiKey = useAppStore((state) => state.hasApiKey);
  const setHasApiKey = useAppStore((state) => state.setHasApiKey);

  const [newApiKey, setNewApiKey] = useState('');
  const [isChangingKey, setIsChangingKey] = useState(false);

  const handleSaveApiKey = async () => {
    if (!newApiKey.trim()) return;

    try {
      // Validate key first
      await invoke('validate_api_key', { apiKey: newApiKey });

      // Save to keyring
      await invoke('set_api_key', { apiKey: newApiKey });

      setHasApiKey(true);
      setNewApiKey('');
      setIsChangingKey(false);

      toast({
        title: t('common.success'),
        description: t('wizard.apiKey.valid'),
      });
    } catch (error) {
      toast({
        variant: 'destructive',
        title: t('common.error'),
        description: String(error),
      });
    }
  };

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <Label>{t('settings.api.key')}</Label>
        {hasApiKey && !isChangingKey ? (
          <div className="flex items-center gap-2">
            <span className="text-sm text-muted-foreground">
              {t('settings.api.keySet')}
            </span>
            <Button variant="outline" size="sm" onClick={() => setIsChangingKey(true)}>
              {t('settings.api.changeKey')}
            </Button>
          </div>
        ) : (
          <div className="flex gap-2">
            <Input
              type="password"
              value={newApiKey}
              onChange={(e) => setNewApiKey(e.target.value)}
              placeholder={t('wizard.apiKey.placeholder')}
            />
            <Button onClick={() => void handleSaveApiKey()} disabled={!newApiKey.trim()}>
              {t('common.save')}
            </Button>
            {isChangingKey && (
              <Button variant="outline" onClick={() => setIsChangingKey(false)}>
                {t('common.cancel')}
              </Button>
            )}
          </div>
        )}
      </div>

      <div className="space-y-2">
        <Label>{t('settings.api.model')}</Label>
        <Select
          value={settings.model}
          onValueChange={(v) => void updateSettings({ model: v })}
        >
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="claude-sonnet-4-20250514">Claude Sonnet 4</SelectItem>
            <SelectItem value="claude-opus-4-5-20251101">Claude Opus 4.5</SelectItem>
            <SelectItem value="claude-haiku-4-5-20251001">Claude Haiku 4.5</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div className="space-y-2">
        <Label>{t('settings.api.maxTokens')}</Label>
        <Input
          type="number"
          value={settings.maxTokens}
          onChange={(e) => void updateSettings({ maxTokens: parseInt(e.target.value) || 4096 })}
          min={1}
          max={64000}
        />
      </div>

      <div className="space-y-2">
        <Label>{t('settings.api.temperature')}</Label>
        <Input
          type="number"
          value={settings.temperature}
          onChange={(e) => void updateSettings({ temperature: parseFloat(e.target.value) || 1.0 })}
          min={0}
          max={2}
          step={0.1}
        />
      </div>

      <div className="space-y-2">
        <Label>{t('settings.api.proxy')}</Label>
        <Input
          value={settings.proxyUrl || ''}
          onChange={(e) => void updateSettings({ proxyUrl: e.target.value || null })}
          placeholder={t('settings.api.proxyPlaceholder')}
        />
      </div>

      <div className="space-y-2">
        <Label>{t('settings.api.timeout')}</Label>
        <Input
          type="number"
          value={settings.apiTimeout}
          onChange={(e) => void updateSettings({ apiTimeout: parseInt(e.target.value) || 120 })}
          min={10}
          max={600}
        />
      </div>
    </div>
  );
}

function PrivacySettings() {
  const { t } = useTranslation();
  const { toast } = useToast();
  const settings = useSettingsStore((state) => state.settings);
  const updateSettings = useSettingsStore((state) => state.updateSettings);

  const handleClearData = async () => {
    if (!window.confirm(t('settings.privacy.clearDataConfirm'))) return;

    try {
      await invoke('clear_all_data');
      toast({
        title: t('common.success'),
        description: 'All data cleared',
      });
      // Reload the page to reset state
      window.location.reload();
    } catch (error) {
      toast({
        variant: 'destructive',
        title: t('common.error'),
        description: String(error),
      });
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <Label>{t('settings.privacy.privacyMode')}</Label>
          <p className="text-sm text-muted-foreground">
            {t('settings.privacy.privacyModeDesc')}
          </p>
        </div>
        <Switch
          checked={settings.privacyMode}
          onCheckedChange={(checked) => void updateSettings({ privacyMode: checked })}
        />
      </div>

      <div className="border-t pt-6">
        <div className="space-y-2">
          <Label className="text-destructive">{t('settings.privacy.clearData')}</Label>
          <p className="text-sm text-muted-foreground">
            {t('settings.privacy.clearDataDesc')}
          </p>
          <Button variant="destructive" onClick={() => void handleClearData()}>
            {t('settings.privacy.clearData')}
          </Button>
        </div>
      </div>
    </div>
  );
}

function AboutSettings() {
  const { t } = useTranslation();

  const handleOpenRepo = () => {
    void open('https://github.com/edward-playground/claude-desktop-linux');
  };

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold">{t('app.name')}</h3>
        <p className="text-sm text-muted-foreground">{t('app.tagline')}</p>
      </div>

      <div className="rounded-lg bg-muted/50 p-4">
        <p className="text-sm">{t('app.description')}</p>
      </div>

      <div className="space-y-2">
        <div className="flex justify-between text-sm">
          <span className="text-muted-foreground">{t('settings.about.version')}</span>
          <span>0.1.0</span>
        </div>
        <div className="flex justify-between text-sm">
          <span className="text-muted-foreground">{t('settings.about.license')}</span>
          <span>MIT</span>
        </div>
        <div className="flex justify-between text-sm">
          <span className="text-muted-foreground">{t('settings.about.repository')}</span>
          <Button variant="link" className="h-auto p-0" onClick={handleOpenRepo}>
            GitHub
          </Button>
        </div>
      </div>
    </div>
  );
}
