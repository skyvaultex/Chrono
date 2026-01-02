import crypto from 'crypto';
import { customAlphabet } from 'nanoid';

// Base32-like alphabet (no confusing chars like 0/O, 1/I/L)
const alphabet = '23456789ABCDEFGHJKMNPQRSTUVWXYZ';
const generateId = customAlphabet(alphabet, 4);

/**
 * Generate a license key
 * Format: PREFIX-XXXX-XXXX-XXXX (e.g., PRO-A2B3-C4D5-E6F7)
 */
export function generateLicenseKey(tier: 'pro' | 'lifetime'): string {
  const prefix = tier === 'lifetime' ? 'LIFE' : 'PRO';
  const parts = [generateId(), generateId(), generateId()];
  return `${prefix}-${parts.join('-')}`;
}

/**
 * Verify LemonSqueezy webhook signature
 */
export function verifyWebhookSignature(
  payload: string,
  signature: string,
  secret: string
): boolean {
  const hmac = crypto.createHmac('sha256', secret);
  const digest = hmac.update(payload).digest('hex');
  return crypto.timingSafeEqual(Buffer.from(signature), Buffer.from(digest));
}

/**
 * Parse tier from LemonSqueezy product/variant name
 */
export function parseTierFromProduct(productName: string, variantName?: string): 'pro' | 'lifetime' {
  const name = (variantName || productName).toLowerCase();
  if (name.includes('lifetime')) return 'lifetime';
  return 'pro';
}

/**
 * Calculate expiry date for subscription
 */
export function calculateExpiryDate(
  renewsAt: string | null,
  endsAt: string | null
): Date | null {
  // If subscription ends (cancelled), use that date
  if (endsAt) return new Date(endsAt);
  // If it renews, add grace period
  if (renewsAt) {
    const date = new Date(renewsAt);
    date.setDate(date.getDate() + 3); // 3 day grace period
    return date;
  }
  return null;
}

/**
 * Check if license is valid (not expired, not revoked)
 */
export function isLicenseValid(license: {
  status: string;
  expires_at: Date | null;
}): boolean {
  if (license.status === 'revoked') return false;
  if (license.status === 'expired') return false;
  if (license.expires_at && new Date(license.expires_at) < new Date()) return false;
  return true;
}

/**
 * Format error response
 */
export function errorResponse(message: string, status = 400) {
  return Response.json({ error: message }, { status });
}

/**
 * Format success response
 */
export function successResponse(data: object, status = 200) {
  return Response.json(data, { status });
}
