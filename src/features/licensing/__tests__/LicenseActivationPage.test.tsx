import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { LicenseActivationPage } from '../LicenseActivationPage';
import { useLicenseStore } from '@/stores/useLicenseStore';

const mockStartTrial = vi.fn();
const mockImportLicense = vi.fn();
const mockGetDeviceId = vi.fn();

vi.mock('@/lib/tauri', () => ({
  licenseApi: {
    startTrial: (...args: unknown[]) => mockStartTrial(...args),
    importLicense: (...args: unknown[]) => mockImportLicense(...args),
    getDeviceId: (...args: unknown[]) => mockGetDeviceId(...args),
    initialize: vi.fn(),
    checkStatus: vi.fn(),
  },
}));

vi.mock('@/lib/config', () => ({
  isTauri: () => false,
}));

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => {
      const map: Record<string, string> = {
        'activation.title': 'Get Started',
        'activation.subtitle': 'Professional invoicing software',
        'activation.description': 'Start your free trial or import a license.',
        'activation.startTrial': 'Start 30-Day Free Trial',
        'activation.or': 'or',
        'activation.importLicense': 'Import License File',
        'activation.deviceId': 'Device ID',
        'activation.copyDeviceId': 'Copy device ID',
        'errors.trialFailed': 'Failed to start trial.',
        'errors.importFailed': 'Failed to import license.',
        'errors.init.unknown': 'An unexpected error occurred.',
      };
      return map[key] ?? key;
    },
  }),
  initReactI18next: { type: '3rdParty', init: () => {} },
}));

describe('LicenseActivationPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockGetDeviceId.mockResolvedValue('ABCD-EF01-2345');
    useLicenseStore.setState({
      status: 'no_license',
      daysRemaining: null,
      expiresAt: null,
      isWriteAllowed: false,
      customerName: null,
      licenseId: null,
      licenseType: null,
      isLoading: false,
      error: null,
    });
  });

  it('renders activation UI with start trial and import buttons', () => {
    render(<LicenseActivationPage />);
    expect(screen.getByText('Get Started')).toBeInTheDocument();
    expect(screen.getByText('Start 30-Day Free Trial')).toBeInTheDocument();
    expect(screen.getByText('Import License File')).toBeInTheDocument();
  });

  it('displays device ID after loading', async () => {
    render(<LicenseActivationPage />);
    await waitFor(() => {
      expect(screen.getByText('ABCD-EF01-2345')).toBeInTheDocument();
    });
  });

  it('calls startTrial API on button click', async () => {
    mockStartTrial.mockResolvedValue({
      status: 'trial_active',
      days_remaining: 30,
      expires_at: null,
      is_write_allowed: true,
      customer_name: null,
      license_id: null,
      license_type: 'trial',
    });

    const user = userEvent.setup();
    render(<LicenseActivationPage />);

    await user.click(screen.getByText('Start 30-Day Free Trial'));
    expect(mockStartTrial).toHaveBeenCalledOnce();
  });

  it('shows error when trial fails', async () => {
    mockStartTrial.mockRejectedValue(new Error('Engine lock failed'));

    const user = userEvent.setup();
    render(<LicenseActivationPage />);

    await user.click(screen.getByText('Start 30-Day Free Trial'));

    await waitFor(() => {
      expect(screen.getByText('Engine lock failed')).toBeInTheDocument();
    });
  });

  it('displays init error from store', () => {
    useLicenseStore.setState({ error: 'unknown' });
    render(<LicenseActivationPage />);
    expect(screen.getByText('An unexpected error occurred.')).toBeInTheDocument();
  });
});
