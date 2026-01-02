import { writable, derived, get } from 'svelte/store';
import type { License, FeatureLimits, LimitCheck } from '../types';
import { Tier } from '../types';
import { getLicense, getFeatureLimits, activateLicense as apiActivateLicense, deactivateLicense as apiDeactivateLicense } from '../api';

// Core stores
export const license = writable<License>({ tier: Tier.Free });
export const featureLimits = writable<FeatureLimits>({
  max_session_types: 2,
  max_goals: 3,
  analytics_days: 7,
  has_invoices: false,
  has_ai_advisor: false,
  has_voice_input: false,
  has_simulator: false,
  has_pdf_export: false,
});

// Derived stores for easy access
export const currentTier = derived(license, $license => $license.tier);
export const isPro = derived(license, $license => 
  $license.tier === Tier.Pro || $license.tier === Tier.Lifetime
);
export const isLifetime = derived(license, $license => $license.tier === Tier.Lifetime);

// Initialize from backend
export async function initializeLicense(): Promise<void> {
  try {
    const [licenseData, limits] = await Promise.all([
      getLicense(),
      getFeatureLimits()
    ]);
    license.set(licenseData);
    featureLimits.set(limits);
  } catch (e) {
    console.error('Failed to initialize license:', e);
    // Keep defaults (Free tier)
  }
}

// Activate a license key
export async function activateLicenseKey(key: string): Promise<{ success: boolean; error?: string }> {
  try {
    const newLicense = await apiActivateLicense(key);
    license.set(newLicense);
    
    // Refresh limits
    const limits = await getFeatureLimits();
    featureLimits.set(limits);
    
    return { success: true };
  } catch (e) {
    return { success: false, error: String(e) };
  }
}

// Deactivate license (return to free)
export async function deactivateLicenseKey(): Promise<void> {
  try {
    const newLicense = await apiDeactivateLicense();
    license.set(newLicense);
    
    const limits = await getFeatureLimits();
    featureLimits.set(limits);
  } catch (e) {
    console.error('Failed to deactivate license:', e);
  }
}

// Helper to check if a feature is available
export function hasFeature(feature: keyof FeatureLimits): boolean {
  const limits = get(featureLimits);
  const value = limits[feature];
  return typeof value === 'boolean' ? value : true;
}

// Tier display helpers
export function getTierDisplayName(tier: Tier): string {
  switch (tier) {
    case Tier.Free: return 'Free';
    case Tier.Pro: return 'Pro';
    case Tier.Lifetime: return 'Lifetime';
    default: return 'Free';
  }
}

export function getTierColor(tier: Tier): string {
  switch (tier) {
    case Tier.Free: return 'text-gray-600';
    case Tier.Pro: return 'text-indigo-600';
    case Tier.Lifetime: return 'text-amber-600';
    default: return 'text-gray-600';
  }
}

export function getTierBadgeClass(tier: Tier): string {
  switch (tier) {
    case Tier.Free: return 'bg-gray-100 text-gray-700';
    case Tier.Pro: return 'bg-indigo-100 text-indigo-700';
    case Tier.Lifetime: return 'bg-amber-100 text-amber-700';
    default: return 'bg-gray-100 text-gray-700';
  }
}
