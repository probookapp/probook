import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Database, CheckCircle, XCircle } from 'lucide-react';
import { dbApi } from '@/lib/tauri';
import { useAuthStore } from '@/stores/useAuthStore';
import { Button, Input } from '@/components/ui';

export function DatabaseSetupPage() {
  const { t } = useTranslation('auth');
  const { setNeedsDbSetup, setLoading } = useAuthStore();
  const [host, setHost] = useState('localhost');
  const [port, setPort] = useState('5432');
  const [database, setDatabase] = useState('probook');
  const [username, setUsername] = useState('postgres');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [testResult, setTestResult] = useState<'success' | 'error' | null>(null);
  const [isTesting, setIsTesting] = useState(false);
  const [isSaving, setIsSaving] = useState(false);

  const getConfig = () => ({
    host,
    port: parseInt(port, 10) || 5432,
    database,
    username,
    password,
  });

  const handleTest = async () => {
    setError('');
    setTestResult(null);
    setIsTesting(true);

    try {
      await dbApi.testConnection(getConfig());
      setTestResult('success');
    } catch (err) {
      setTestResult('error');
      setError(typeof err === 'string' ? err : t('database.connectionFailed'));
    } finally {
      setIsTesting(false);
    }
  };

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setIsSaving(true);

    try {
      await dbApi.saveConfig(getConfig());
      setNeedsDbSetup(false);
      setLoading(true);
      // Re-trigger auth initialization
      window.location.reload();
    } catch (err) {
      setError(typeof err === 'string' ? err : t('database.connectionFailed'));
    } finally {
      setIsSaving(false);
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
          <p className="text-gray-500 dark:text-gray-400 mt-2">{t('database.subtitle')}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-8">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2 flex items-center gap-2">
            <Database className="h-5 w-5" />
            {t('database.title')}
          </h2>
          <p className="text-sm text-gray-500 dark:text-gray-400 mb-6">
            {t('database.instructions')}
          </p>

          <form onSubmit={handleSave} className="space-y-4">
            <div className="grid grid-cols-3 gap-3">
              <div className="col-span-2">
                <Input
                  label={t('database.host')}
                  value={host}
                  onChange={(e) => setHost(e.target.value)}
                  required
                />
              </div>
              <Input
                label={t('database.port')}
                value={port}
                onChange={(e) => setPort(e.target.value)}
                required
              />
            </div>

            <Input
              label={t('database.databaseName')}
              value={database}
              onChange={(e) => setDatabase(e.target.value)}
              required
            />

            <Input
              label={t('database.username')}
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              autoComplete="username"
              required
            />

            <Input
              label={t('database.password')}
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              autoComplete="current-password"
            />

            {testResult === 'success' && (
              <div className="flex items-center gap-2 text-green-600 dark:text-green-400 text-sm">
                <CheckCircle className="h-4 w-4" />
                {t('database.connectionSuccess')}
              </div>
            )}

            {testResult === 'error' && (
              <div className="flex items-center gap-2 text-red-600 dark:text-red-400 text-sm">
                <XCircle className="h-4 w-4" />
                {error || t('database.connectionFailed')}
              </div>
            )}

            {!testResult && error && (
              <p className="text-sm text-red-600 dark:text-red-400">{error}</p>
            )}

            <div className="flex gap-3 pt-2">
              <Button
                type="button"
                variant="secondary"
                onClick={handleTest}
                isLoading={isTesting}
                className="flex-1"
              >
                {t('database.testConnection')}
              </Button>
              <Button
                type="submit"
                isLoading={isSaving}
                className="flex-1"
              >
                <Database className="h-4 w-4 mr-2" />
                {t('database.saveAndConnect')}
              </Button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
