import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { AlertTriangle, X } from 'lucide-react';
import { useLicenseStore } from '@/stores/useLicenseStore';

export function LicenseWarningBanner() {
  const { t } = useTranslation('licensing');
  const { status, daysRemaining } = useLicenseStore();
  const [dismissed, setDismissed] = useState(() => sessionStorage.getItem('license-banner-dismissed') === 'true');

  // Show banner during grace period or when trial is ending soon (<=7 days)
  const isGracePeriod = status === 'grace_period';
  const isTrialEndingSoon =
    status === 'trial_active' && daysRemaining !== null && daysRemaining <= 7;

  if (dismissed || (!isGracePeriod && !isTrialEndingSoon)) {
    return null;
  }

  const message = isGracePeriod
    ? t('banner.graceMessage', { days: daysRemaining })
    : t('banner.trialEndingMessage', { days: daysRemaining });

  return (
    <div className="bg-amber-50 dark:bg-amber-900/30 border-b border-amber-200 dark:border-amber-800 px-4 py-2.5 flex items-center gap-3">
      <AlertTriangle className="h-4 w-4 text-amber-600 dark:text-amber-400 shrink-0" />
      <p className="text-sm text-amber-700 dark:text-amber-300 flex-1">
        {message}
      </p>
      <button
        onClick={() => { setDismissed(true); sessionStorage.setItem('license-banner-dismissed', 'true'); }}
        className="p-1 text-amber-500 hover:text-amber-700 dark:hover:text-amber-200 transition-colors"
      >
        <X className="h-4 w-4" />
      </button>
    </div>
  );
}
