import { neon } from '@neondatabase/serverless';

// Create SQL query function from connection string
const sql = neon(process.env.DATABASE_URL!);

// License tiers
export type Tier = 'free' | 'pro' | 'lifetime';

// License status
export type LicenseStatus = 'active' | 'expired' | 'revoked';

// License record
export interface License {
  id: string;
  license_key: string;
  tier: Tier;
  status: LicenseStatus;
  email: string | null;
  customer_id: string | null;
  order_id: string | null;
  subscription_id: string | null;
  max_activations: number;
  activated_at: Date | null;
  expires_at: Date | null;
  created_at: Date;
  updated_at: Date;
}

// Device activation record
export interface LicenseActivation {
  id: string;
  license_id: string;
  device_id: string;
  device_name: string | null;
  activated_at: Date;
}

// Webhook event record (for idempotency)
export interface WebhookEvent {
  id: string;
  event_id: string;
  event_type: string;
  processed_at: Date;
}

// Feature limits per tier
export interface FeatureLimits {
  max_session_types: number | null;
  max_goals: number | null;
  analytics_days: number | null;
  has_invoices: boolean;
  has_ai_advisor: boolean;
  has_voice_input: boolean;
  has_simulator: boolean;
  has_pdf_export: boolean;
}

export function getFeatureLimits(tier: Tier): FeatureLimits {
  switch (tier) {
    case 'free':
      return {
        max_session_types: 2,
        max_goals: 3,
        analytics_days: 7,
        has_invoices: false,
        has_ai_advisor: true,  // Free gets 10/day, Pro gets 100/day
        has_voice_input: false,
        has_simulator: false,
        has_pdf_export: false,
      };
    case 'pro':
    case 'lifetime':
      return {
        max_session_types: null,
        max_goals: null,
        analytics_days: null,
        has_invoices: true,
        has_ai_advisor: true,
        has_voice_input: true,
        has_simulator: true,
        has_pdf_export: true,
      };
  }
}

// Database operations
export const db = {
  // ========== LICENSES ==========
  
  async createLicense(data: {
    license_key: string;
    tier: Tier;
    email?: string;
    customer_id?: string;
    order_id?: string;
    subscription_id?: string;
    max_activations?: number;
    expires_at?: Date | null;
  }): Promise<License> {
    const result = await sql`
      INSERT INTO licenses (
        license_key, tier, email, customer_id, order_id, subscription_id, max_activations, expires_at
      ) VALUES (
        ${data.license_key},
        ${data.tier},
        ${data.email || null},
        ${data.customer_id || null},
        ${data.order_id || null},
        ${data.subscription_id || null},
        ${data.max_activations || 3},
        ${data.expires_at?.toISOString() || null}
      )
      RETURNING *
    `;
    return result[0] as License;
  },

  async getLicenseByKey(license_key: string): Promise<License | null> {
    const result = await sql`
      SELECT * FROM licenses WHERE license_key = ${license_key}
    `;
    return (result[0] as License) || null;
  },

  async getLicenseById(id: string): Promise<License | null> {
    const result = await sql`
      SELECT * FROM licenses WHERE id = ${id}
    `;
    return (result[0] as License) || null;
  },

  async getLicenseByOrderId(order_id: string): Promise<License | null> {
    const result = await sql`
      SELECT * FROM licenses WHERE order_id = ${order_id}
    `;
    return (result[0] as License) || null;
  },

  async getLicenseBySubscriptionId(subscription_id: string): Promise<License | null> {
    const result = await sql`
      SELECT * FROM licenses WHERE subscription_id = ${subscription_id}
    `;
    return (result[0] as License) || null;
  },

  async updateLicenseStatus(id: string, status: LicenseStatus): Promise<void> {
    await sql`
      UPDATE licenses SET status = ${status}, updated_at = NOW() WHERE id = ${id}
    `;
  },

  async updateLicenseExpiry(id: string, expires_at: Date | null): Promise<void> {
    await sql`
      UPDATE licenses SET expires_at = ${expires_at?.toISOString() || null}, updated_at = NOW() WHERE id = ${id}
    `;
  },

  async searchLicenses(query: string, limit = 50): Promise<License[]> {
    const searchPattern = `%${query}%`;
    const result = await sql`
      SELECT * FROM licenses 
      WHERE license_key ILIKE ${searchPattern} 
         OR email ILIKE ${searchPattern}
      ORDER BY created_at DESC
      LIMIT ${limit}
    `;
    return result as License[];
  },

  async getAllLicenses(limit = 100, offset = 0): Promise<License[]> {
    const result = await sql`
      SELECT * FROM licenses ORDER BY created_at DESC LIMIT ${limit} OFFSET ${offset}
    `;
    return result as License[];
  },

  // ========== ACTIVATIONS ==========

  async getActivations(license_id: string): Promise<LicenseActivation[]> {
    const result = await sql`
      SELECT * FROM license_activations WHERE license_id = ${license_id} ORDER BY activated_at DESC
    `;
    return result as LicenseActivation[];
  },

  async getActivationCount(license_id: string): Promise<number> {
    const result = await sql`
      SELECT COUNT(*) as count FROM license_activations WHERE license_id = ${license_id}
    `;
    return parseInt(result[0].count as string);
  },

  async addActivation(license_id: string, device_id: string, device_name?: string): Promise<LicenseActivation> {
    const result = await sql`
      INSERT INTO license_activations (license_id, device_id, device_name)
      VALUES (${license_id}, ${device_id}, ${device_name || null})
      ON CONFLICT (license_id, device_id) DO UPDATE SET activated_at = NOW()
      RETURNING *
    `;
    return result[0] as LicenseActivation;
  },

  async removeActivation(license_id: string, device_id: string): Promise<void> {
    await sql`
      DELETE FROM license_activations WHERE license_id = ${license_id} AND device_id = ${device_id}
    `;
  },

  async removeAllActivations(license_id: string): Promise<void> {
    await sql`
      DELETE FROM license_activations WHERE license_id = ${license_id}
    `;
  },

  // ========== WEBHOOK EVENTS ==========

  async isEventProcessed(event_id: string): Promise<boolean> {
    const result = await sql`
      SELECT 1 FROM webhook_events WHERE event_id = ${event_id}
    `;
    return result.length > 0;
  },

  async markEventProcessed(event_id: string, event_type: string): Promise<void> {
    await sql`
      INSERT INTO webhook_events (event_id, event_type) VALUES (${event_id}, ${event_type})
      ON CONFLICT (event_id) DO NOTHING
    `;
  },
};
