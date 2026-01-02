require('dotenv').config({ path: '.env.local' });
const { neon } = require('@neondatabase/serverless');

async function checkLicenses() {
  const sql = neon(process.env.DATABASE_URL);
  const licenses = await sql`SELECT license_key, tier, email, status, created_at FROM licenses ORDER BY created_at DESC LIMIT 5`;
  console.log('📋 Recent Licenses:');
  if (licenses.length === 0) {
    console.log('   (none yet - waiting for first purchase)');
  } else {
    licenses.forEach(l => {
      console.log(`   ${l.license_key} | ${l.tier} | ${l.email} | ${l.status}`);
    });
  }
}

checkLicenses();
