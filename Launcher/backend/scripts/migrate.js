// Migration script - run with: npm run db:migrate
require('dotenv').config({ path: '.env.local' });
const { neon } = require('@neondatabase/serverless');

async function migrate() {
  console.log('🔄 Running migrations...');
  
  if (!process.env.DATABASE_URL) {
    console.error('❌ DATABASE_URL not set. Create .env.local first.');
    process.exit(1);
  }
  
  const sql = neon(process.env.DATABASE_URL);
  
  try {
    // Enable UUID extension (Neon has it by default, but just in case)
    console.log('Ensuring UUID extension...');
    try {
      await sql`CREATE EXTENSION IF NOT EXISTS "uuid-ossp"`;
    } catch (e) {
      // Extension may already exist
      console.log('  UUID extension already exists');
    }
    
    // Create licenses table
    console.log('Creating licenses table...');
    await sql`
      CREATE TABLE IF NOT EXISTS licenses (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        license_key TEXT UNIQUE NOT NULL,
        tier TEXT NOT NULL CHECK (tier IN ('free', 'pro', 'lifetime')),
        status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'expired', 'revoked')),
        email TEXT,
        customer_id TEXT,
        order_id TEXT UNIQUE,
        subscription_id TEXT,
        max_activations INTEGER NOT NULL DEFAULT 3,
        activated_at TIMESTAMPTZ,
        expires_at TIMESTAMPTZ,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
      )
    `;
    
    // Create indexes
    console.log('Creating indexes...');
    await sql`CREATE INDEX IF NOT EXISTS idx_licenses_key ON licenses(license_key)`;
    await sql`CREATE INDEX IF NOT EXISTS idx_licenses_email ON licenses(email)`;
    await sql`CREATE INDEX IF NOT EXISTS idx_licenses_status ON licenses(status)`;
    await sql`CREATE INDEX IF NOT EXISTS idx_licenses_subscription ON licenses(subscription_id)`;
    
    // Create activations table
    console.log('Creating license_activations table...');
    await sql`
      CREATE TABLE IF NOT EXISTS license_activations (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        license_id UUID NOT NULL REFERENCES licenses(id) ON DELETE CASCADE,
        device_id TEXT NOT NULL,
        device_name TEXT,
        activated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        UNIQUE(license_id, device_id)
      )
    `;
    await sql`CREATE INDEX IF NOT EXISTS idx_activations_license ON license_activations(license_id)`;
    
    // Create webhook events table
    console.log('Creating webhook_events table...');
    await sql`
      CREATE TABLE IF NOT EXISTS webhook_events (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        event_id TEXT UNIQUE NOT NULL,
        event_type TEXT NOT NULL,
        processed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
      )
    `;
    await sql`CREATE INDEX IF NOT EXISTS idx_webhook_events_id ON webhook_events(event_id)`;
    
    console.log('✅ Migrations complete!');
  } catch (error) {
    console.error('❌ Migration failed:', error);
    process.exit(1);
  }
}

migrate();migrate();
