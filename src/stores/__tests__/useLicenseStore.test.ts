import { describe, it, expect, beforeEach } from 'vitest';
import { useLicenseStore } from '@/stores/useLicenseStore';
import type { LicenseStatusInfo } from '@/types';

describe('useLicenseStore', () => {
  beforeEach(() => {
    useLicenseStore.setState({
      status: 'loading',
      daysRemaining: null,
      expiresAt: null,
      isWriteAllowed: false,
      customerName: null,
      licenseId: null,
      licenseType: null,
      isLoading: true,
      error: null,
    });
  });

  it('has correct initial state', () => {
    const state = useLicenseStore.getState();
    expect(state.status).toBe('loading');
    expect(state.daysRemaining).toBeNull();
    expect(state.expiresAt).toBeNull();
    expect(state.isWriteAllowed).toBe(false);
    expect(state.customerName).toBeNull();
    expect(state.licenseId).toBeNull();
    expect(state.licenseType).toBeNull();
    expect(state.isLoading).toBe(true);
    expect(state.error).toBeNull();
  });

  it('setLicenseInfo maps snake_case fields to camelCase', () => {
    const info: LicenseStatusInfo = {
      status: 'trial_active',
      days_remaining: 25,
      expires_at: '2026-03-15T00:00:00Z',
      is_write_allowed: true,
      customer_name: 'Test Customer',
      license_id: 'LIC-123',
      license_type: 'trial',
    };

    useLicenseStore.getState().setLicenseInfo(info);
    const state = useLicenseStore.getState();

    expect(state.status).toBe('trial_active');
    expect(state.daysRemaining).toBe(25);
    expect(state.expiresAt).toBe('2026-03-15T00:00:00Z');
    expect(state.isWriteAllowed).toBe(true);
    expect(state.customerName).toBe('Test Customer');
    expect(state.licenseId).toBe('LIC-123');
    expect(state.licenseType).toBe('trial');
    expect(state.isLoading).toBe(false);
  });

  it('setLicenseInfo clears existing error', () => {
    useLicenseStore.getState().setError('some error');
    expect(useLicenseStore.getState().error).toBe('some error');

    const info: LicenseStatusInfo = {
      status: 'licensed',
      days_remaining: 365,
      expires_at: '2027-02-27T00:00:00Z',
      is_write_allowed: true,
      customer_name: null,
      license_id: null,
      license_type: 'annual',
    };

    useLicenseStore.getState().setLicenseInfo(info);
    expect(useLicenseStore.getState().error).toBeNull();
  });

  it('setLicenseInfo with expired status sets isWriteAllowed false', () => {
    const info: LicenseStatusInfo = {
      status: 'expired',
      days_remaining: null,
      expires_at: null,
      is_write_allowed: false,
      customer_name: null,
      license_id: null,
      license_type: null,
    };

    useLicenseStore.getState().setLicenseInfo(info);
    const state = useLicenseStore.getState();

    expect(state.status).toBe('expired');
    expect(state.isWriteAllowed).toBe(false);
  });

  it('setLoading toggles loading state', () => {
    useLicenseStore.getState().setLoading(false);
    expect(useLicenseStore.getState().isLoading).toBe(false);

    useLicenseStore.getState().setLoading(true);
    expect(useLicenseStore.getState().isLoading).toBe(true);
  });

  it('setError stores error string', () => {
    useLicenseStore.getState().setError('network');
    expect(useLicenseStore.getState().error).toBe('network');
  });

  it('setError clears error with null', () => {
    useLicenseStore.getState().setError('engine');
    expect(useLicenseStore.getState().error).toBe('engine');

    useLicenseStore.getState().setError(null);
    expect(useLicenseStore.getState().error).toBeNull();
  });
});
