import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { KeyRound, Upload, Copy, Check } from 'lucide-react';
import { licenseApi } from '@/lib/tauri';
import { useLicenseStore } from '@/stores/useLicenseStore';
import { Button } from '@/components/ui';
import { isTauri } from '@/lib/config';

export function LicenseSettingsSection() {
  const { t } = useTranslation('licensing');
  const { status, daysRemaining, expiresAt, customerName, licenseId, licenseType, setLicenseInfo, error: initError } =
    useLicenseStore();
  const [deviceId, setDeviceId] = useState('');
  const [copied, setCopied] = useState(false);
  const [error, setError] = useState('');
  const [isImporting, setIsImporting] = useState(false);

  useEffect(() => {
    licenseApi.getDeviceId().then(setDeviceId).catch(() => {});
  }, []);

  const handleImportLicense = async () => {
    setError('');
    try {
      let fileOrPath: string | File;

      if (isTauri()) {
        const { open } = await import('@tauri-apps/plugin-dialog');
        const filePath = await open({
          multiple: false,
          filters: [{ name: 'Probook License', extensions: ['probook'] }],
        });
        if (!filePath) return;
        fileOrPath = filePath as string;
      } else {
        const file = await new Promise<File | null>((resolve) => {
          const input = document.createElement('input');
          input.type = 'file';
          input.accept = '.probook';
          input.onchange = () => resolve(input.files?.[0] ?? null);
          input.click();
        });
        if (!file) return;
        fileOrPath = file;
      }

      setIsImporting(true);
      const info = await licenseApi.importLicense(fileOrPath);
      setLicenseInfo(info);
    } catch (err) {
      setError(err instanceof Error ? err.message : typeof err === 'string' ? err : t('errors.importFailed'));
    } finally {
      setIsImporting(false);
    }
  };

  const handleCopyDeviceId = async () => {
    try {
      await navigator.clipboard.writeText(deviceId);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // Fallback
    }
  };

  const statusBadge = () => {
    switch (status) {
      case 'trial_active':
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900/50 dark:text-blue-300">
            {t('status.trial')} — {daysRemaining} {t('status.daysLeft')}
          </span>
        );
      case 'licensed':
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-900/50 dark:text-green-300">
            {t('status.active')}
          </span>
        );
      case 'grace_period':
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-amber-100 text-amber-800 dark:bg-amber-900/50 dark:text-amber-300">
            {t('status.gracePeriod')} — {daysRemaining} {t('status.daysLeft')}
          </span>
        );
      case 'expired':
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900/50 dark:text-red-300">
            {t('status.expired')}
          </span>
        );
      default:
        return null;
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-2">
        <KeyRound className="h-5 w-5 text-gray-500" />
        <h3 className="text-lg font-medium text-gray-900 dark:text-gray-100">
          {t('settings.title')}
        </h3>
        {statusBadge()}
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 text-sm">
        {licenseType && (
          <div>
            <p className="text-gray-500 dark:text-gray-400">{t('settings.licenseType')}</p>
            <p className="font-medium text-gray-900 dark:text-gray-100">
              {licenseType === 'trial' ? t('status.trial') : licenseType === 'annual' ? t('status.annual', 'Annual') : licenseType === 'lifetime' ? t('status.lifetime', 'Lifetime') : licenseType}
            </p>
          </div>
        )}
        {customerName && (
          <div>
            <p className="text-gray-500 dark:text-gray-400">{t('settings.customer')}</p>
            <p className="font-medium text-gray-900 dark:text-gray-100">{customerName}</p>
          </div>
        )}
        {expiresAt && (
          <div>
            <p className="text-gray-500 dark:text-gray-400">{t('settings.expiresAt')}</p>
            <p className="font-medium text-gray-900 dark:text-gray-100">
              {new Date(expiresAt).toLocaleDateString()}
            </p>
          </div>
        )}
        {licenseId && (
          <div>
            <p className="text-gray-500 dark:text-gray-400">{t('settings.licenseId')}</p>
            <p className="font-mono text-xs text-gray-600 dark:text-gray-300">{licenseId}</p>
          </div>
        )}
      </div>

      <div>
        <p className="text-xs text-gray-400 dark:text-gray-500 mb-1">
          {t('settings.deviceId')}
        </p>
        <div className="flex items-center gap-2">
          <code className="text-sm font-mono text-gray-600 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 px-3 py-1.5 rounded">
            {deviceId}
          </code>
          <button
            onClick={handleCopyDeviceId}
            className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
          >
            {copied ? (
              <Check className="h-4 w-4 text-green-500" />
            ) : (
              <Copy className="h-4 w-4" />
            )}
          </button>
        </div>
      </div>

      <div className="flex gap-3 pt-2">
        <Button
          variant="secondary"
          onClick={handleImportLicense}
          isLoading={isImporting}
          size="sm"
        >
          <Upload className="h-4 w-4 mr-2" />
          {t('settings.importLicense')}
        </Button>
      </div>

      {initError && (
        <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3">
          <p className="text-sm text-red-700 dark:text-red-300">
            {t(`errors.init.${initError}`, t('errors.init.unknown'))}
          </p>
        </div>
      )}

      {error && (
        <p className="text-sm text-red-600 dark:text-red-400">{error}</p>
      )}
    </div>
  );
}
