require('dotenv').config({ path: '.env.local' });
const { neon } = require('@neondatabase/serverless');

async function check() {
  const sql = neon(process.env.DATABASE_URL);
  const tables = await sql`SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'`;
  console.log('✅ Tables in database:', tables.map(t => t.table_name));
}

check();
