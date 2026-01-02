// Script to manually add a license to the database
// Run with: node scripts/add-license.js

require('dotenv').config({ path: '.env.local' });
const { neon } = require('@neondatabase/serverless');

const sql = neon(process.env.DATABASE_URL);

async function addLicense() {
  const licenseKey = process.argv[2] || '89ED66B2-E969-4041-9644-6C27E8BE50B9';
  const tier = process.argv[3] || 'pro';
  const email = process.argv[4] || 'maxym@example.com';
  
  console.log('Adding license:', licenseKey);
  console.log('Tier:', tier);
  console.log('Email:', email);
  
  try {
    // Check if license already exists
    const existing = await sql`SELECT * FROM licenses WHERE license_key = ${licenseKey}`;
    
    if (existing.length > 0) {
      console.log('License already exists:', existing[0]);
      return;
    }
    
    // Insert new license
    const result = await sql`
      INSERT INTO licenses (
        license_key, tier, email, max_activations, status
      ) VALUES (
        ${licenseKey},
        ${tier},
        ${email},
        3,
        'active'
      )
      RETURNING *
    `;
    
    console.log('License created successfully!');
    console.log(result[0]);
  } catch (error) {
    console.error('Error:', error.message);
  }
}

addLicense();
