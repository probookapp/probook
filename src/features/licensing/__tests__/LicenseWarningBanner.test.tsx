import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { LicenseWarningBanner } from '../LicenseWarningBanner';
import { useLicenseStore } from '@/stores/useLicenseStore';

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string, opts?: Record<string, unknown>) => {
      if (key === 'banner.graceMessage') return `Grace: ${opts?.days} days left`;
      if (key === 'banner.trialEndingMessage') return `Trial ends in ${opts?.days} days`;
      return key;
    },
  }),
}));

describe('LicenseWarningBanner', () => {
  beforeEach(() => {
    sessionStorage.clear();
    useLicenseStore.setState({
      status: 'licensed',
      daysRemaining: 365,
      expiresAt: null,
      isWriteAllowed: true,
      customerName: null,
      licenseId: null,
      licenseType: null,
      isLoading: false,
      error: null,
    });
  });

  it('is hidden when status is licensed', () => {
    const { container } = render(<LicenseWarningBanner />);
    expect(container.innerHTML).toBe('');
  });

  it('is hidden when trial has more than 7 days', () => {
    useLicenseStore.setState({ status: 'trial_active', daysRemaining: 20 });
    const { container } = render(<LicenseWarningBanner />);
    expect(container.innerHTML).toBe('');
  });

  it('is visible when trial has 7 or fewer days', () => {
    useLicenseStore.setState({ status: 'trial_active', daysRemaining: 5 });
    render(<LicenseWarningBanner />);
    expect(screen.getByText('Trial ends in 5 days')).toBeInTheDocument();
  });

  it('is visible during grace period', () => {
    useLicenseStore.setState({ status: 'grace_period', daysRemaining: 10 });
    render(<LicenseWarningBanner />);
    expect(screen.getByText('Grace: 10 days left')).toBeInTheDocument();
  });

  it('can be dismissed', async () => {
    const user = userEvent.setup();
    useLicenseStore.setState({ status: 'grace_period', daysRemaining: 10 });
    render(<LicenseWarningBanner />);

    expect(screen.getByText('Grace: 10 days left')).toBeInTheDocument();

    const dismissButton = screen.getByRole('button');
    await user.click(dismissButton);

    expect(screen.queryByText('Grace: 10 days left')).not.toBeInTheDocument();
    expect(sessionStorage.getItem('license-banner-dismissed')).toBe('true');
  });

  it('stays hidden if previously dismissed in session', () => {
    sessionStorage.setItem('license-banner-dismissed', 'true');
    useLicenseStore.setState({ status: 'grace_period', daysRemaining: 10 });
    const { container } = render(<LicenseWarningBanner />);
    expect(container.innerHTML).toBe('');
  });
});
