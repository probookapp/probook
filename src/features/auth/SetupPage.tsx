import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { ShieldCheck, Eye, EyeOff } from 'lucide-react';
import { authApi } from '@/lib/tauri';
import { useAuthStore } from '@/stores/useAuthStore';
import { Button, Input } from '@/components/ui';

export function SetupPage() {
  const { t } = useTranslation('auth');
  const { setUser } = useAuthStore();
  const [username, setUsername] = useState('');
  const [displayName, setDisplayName] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [error, setError] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    if (password !== confirmPassword) {
      setError(t('setup.passwordMismatch'));
      return;
    }

    if (password.length < 8) {
      setError(t('setup.passwordTooShort'));
      return;
    }

    setIsLoading(true);

    try {
      const user = await authApi.setupAdmin({
        username,
        display_name: displayName,
        password,
      });
      setUser(user);
    } catch (err) {
      setError(typeof err === 'string' ? err : t('setup.error'));
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-100 dark:bg-gray-900 p-4">
      <div className="w-full max-w-md">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-gray-100">Probook</h1>
          <p className="text-gray-500 dark:text-gray-400 mt-2">{t('setup.subtitle')}</p>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-8">
          <div className="flex items-center gap-3 mb-6">
            <ShieldCheck className="h-6 w-6 text-primary-600" />
            <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100">
              {t('setup.title')}
            </h2>
          </div>

          <p className="text-sm text-gray-500 dark:text-gray-400 mb-6">
            {t('setup.description')}
          </p>

          <form onSubmit={handleSubmit} className="space-y-4">
            <Input
              label={t('setup.username')}
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              autoComplete="username"
              autoFocus
              required
            />

            <Input
              label={t('setup.displayName')}
              value={displayName}
              onChange={(e) => setDisplayName(e.target.value)}
              autoComplete="name"
              required
            />

            <div className="relative">
              <Input
                label={t('setup.password')}
                type={showPassword ? 'text' : 'password'}
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                autoComplete="new-password"
                required
              />
              <button
                type="button"
                onClick={() => setShowPassword(!showPassword)}
                className="absolute right-3 top-8 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
              >
                {showPassword ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
              </button>
            </div>

            <Input
              label={t('setup.confirmPassword')}
              type={showPassword ? 'text' : 'password'}
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              autoComplete="new-password"
              required
            />

            {error && (
              <p className="text-sm text-red-600 dark:text-red-400">{error}</p>
            )}

            <Button type="submit" className="w-full" isLoading={isLoading}>
              <ShieldCheck className="h-4 w-4 mr-2" />
              {t('setup.submit')}
            </Button>
          </form>
        </div>
      </div>
    </div>
  );
}
