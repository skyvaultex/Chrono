# Chrono API - License Management Backend

## Setup

1. Install dependencies:
```bash
npm install
```

2. Set up Neon Postgres:
   - Go to Vercel Dashboard → Storage → Browse Marketplace → Neon
   - Create a new project and database
   - Copy the connection string (pooled recommended)

3. Create `.env.local`:
```env
# Database (Neon Serverless Postgres)
DATABASE_URL="postgres://user:password@ep-xxx.us-east-1.aws.neon.tech/neondb?sslmode=require"

# LemonSqueezy
LEMONSQUEEZY_API_KEY="your-api-key"
LEMONSQUEEZY_WEBHOOK_SECRET="your-webhook-secret"
LEMONSQUEEZY_STORE_ID="your-store-id"

# Resend (Email)
RESEND_API_KEY="re_xxxxx"
FROM_EMAIL="Chrono <hello@chrono.app>"

# Admin
ADMIN_TOKEN="generate-a-random-secret"

# App
APP_URL="https://chrono.app"
```

4. Run migrations:
```bash
npm run db:migrate
```

5. Deploy to Vercel:
```bash
vercel
```

## API Endpoints

### Webhooks
- `POST /api/webhooks/lemonsqueezy` - Handle purchase events

### License
- `POST /api/license/validate` - Validate a license key
- `POST /api/license/activate` - Activate on a device
- `POST /api/license/deactivate` - Deactivate from a device

### Admin (protected)
- `GET /api/admin/licenses` - List/search licenses
- `GET /api/admin/licenses/[id]` - Get license details
- `POST /api/admin/licenses/[id]/revoke` - Revoke a license

## LemonSqueezy Setup

1. Create products:
   - **Chrono Pro Monthly** - $5/month subscription
   - **Chrono Pro Yearly** - $39/year subscription
   - **Chrono Lifetime** - $79 one-time

2. Enable license keys in each product:
   - Product → License Keys → Enable
   - Set activation limit (e.g., 3 devices)

3. Set up webhook:
   - Settings → Webhooks → Add Endpoint
   - URL: `https://your-domain.vercel.app/api/webhooks/lemonsqueezy`
   - Events: `order_created`, `subscription_created`, `subscription_updated`, `subscription_cancelled`, `order_refunded`

## Security Notes

- Webhook signature is verified on every request
- Admin endpoints require `x-admin-token` header
- License keys are stored plain (not hashed) for easy support
- Device IDs are random UUIDs generated client-side
- Rate limiting is handled at Vercel edge level
