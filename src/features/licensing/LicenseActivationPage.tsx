import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { KeyRound, Play, Upload, Copy, Check } from 'lucide-react';
import { licenseApi } from '@/lib/tauri';
import { useLicenseStore } from '@/stores/useLicenseStore';
import { Button } from '@/components/ui';
import { isTauri } from '@/lib/config';

export function LicenseActivationPage() {
  const { t } = useTranslation('licensing');
  const { setLicenseInfo, error: initError } = useLicenseStore();
  const [deviceId, setDeviceId] = useState('');
  const [error, setError] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    licenseApi.getDeviceId().then(setDeviceId).catch(() => {});
  }, []);

  const handleStartTrial = async () => {
    setError('');
    setIsLoading(true);
    try {
      const info = await licenseApi.startTrial();
      setLicenseInfo(info);
    } catch (err) {
      setError(err instanceof Error ? err.message : typeof err === 'string' ? err : t('errors.trialFailed'));
    } finally {
      setIsLoading(false);
    }
  };

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

      setIsLoading(true);
      const info = await licenseApi.importLicense(fileOrPath);
      setLicenseInfo(info);
    } catch (err) {
      setError(err instanceof Error ? err.message : typeof err === 'string' ? err : t('errors.importFailed'));
    } finally {
      setIsLoading(false);
    }
  };

  const handleCopyDeviceId = async () => {
    try {
      await navigator.clipboard.writeText(deviceId);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // Fallback for environments without clipboard API
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-100 dark:bg-gray-900 p-4">
      <div className="w-full max-w-md">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-gray-100 flex items-center justify-center gap-2">
            <img src="/probook-icon.png" alt="Probook" className="h-9 w-9" />
            Probook
          </h1>
          <p className="text-gray-500 dark:text-gray-400 mt-2">
            {t('activation.subtitle')}
          </p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-8">
          <div className="flex items-center gap-3 mb-6">
            <KeyRound className="h-6 w-6 text-primary-600" />
            <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100">
              {t('activation.title')}
            </h2>
          </div>

          <p className="text-sm text-gray-500 dark:text-gray-400 mb-6">
            {t('activation.description')}
          </p>

          {initError && (
            <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3 mb-4">
              <p className="text-sm text-red-700 dark:text-red-300">
                {t(`errors.init.${initError}`, t('errors.init.unknown'))}
              </p>
            </div>
          )}

          <div className="space-y-4">
            <Button
              onClick={handleStartTrial}
              className="w-full"
              isLoading={isLoading}
            >
              <Play className="h-4 w-4 mr-2" />
              {t('activation.startTrial')}
            </Button>

            <div className="relative flex items-center justify-center">
              <div className="absolute inset-0 flex items-center">
                <div className="w-full border-t border-gray-200 dark:border-gray-700" />
              </div>
              <span className="relative bg-white dark:bg-gray-800 px-4 text-sm text-gray-400">
                {t('activation.or')}
              </span>
            </div>

            <Button
              variant="secondary"
              onClick={handleImportLicense}
              className="w-full"
              disabled={isLoading}
            >
              <Upload className="h-4 w-4 mr-2" />
              {t('activation.importLicense')}
            </Button>
          </div>

          {error && (
            <p className="text-sm text-red-600 dark:text-red-400 mt-4">{error}</p>
          )}

          {deviceId && (
            <div className="mt-6 pt-4 border-t border-gray-200 dark:border-gray-700">
              <p className="text-xs text-gray-400 dark:text-gray-500 mb-1">
                {t('activation.deviceId')}
              </p>
              <div className="flex items-center gap-2">
                <code className="text-sm font-mono text-gray-600 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 px-3 py-1.5 rounded flex-1">
                  {deviceId}
                </code>
                <button
                  onClick={handleCopyDeviceId}
                  className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
                  title={t('activation.copyDeviceId')}
                >
                  {copied ? (
                    <Check className="h-4 w-4 text-green-500" />
                  ) : (
                    <Copy className="h-4 w-4" />
                  )}
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
