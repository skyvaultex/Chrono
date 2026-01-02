import { NextRequest } from 'next/server';
import { db } from '@/lib/db';
import { 
  verifyWebhookSignature, 
  generateLicenseKey, 
  parseTierFromProduct,
  calculateExpiryDate,
  errorResponse,
  successResponse
} from '@/lib/utils';
import { sendLicenseEmail, sendRefundEmail } from '@/lib/email';

// LemonSqueezy webhook events we care about
type WebhookEvent = 
  | 'order_created'
  | 'subscription_created'
  | 'subscription_updated'
  | 'subscription_cancelled'
  | 'subscription_payment_success'
  | 'subscription_payment_failed'
  | 'order_refunded';

interface WebhookPayload {
  meta: {
    event_name: WebhookEvent;
    custom_data?: {
      device_id?: string;
    };
  };
  data: {
    id: string;
    attributes: {
      order_number?: number;
      status: string;
      user_email: string;
      user_name: string;
      first_order_item?: {
        product_id: number;
        product_name: string;
        variant_id: number;
        variant_name: string;
      };
      // For subscriptions
      product_id?: number;
      product_name?: string;
      variant_id?: number;
      variant_name?: string;
      renews_at?: string;
      ends_at?: string;
      // For orders
      customer_id?: number;
    };
  };
}

export async function POST(request: NextRequest) {
  try {
    // Get raw body for signature verification
    const rawBody = await request.text();
    const signature = request.headers.get('x-signature') || '';
    
    // Verify webhook signature
    const secret = process.env.LEMONSQUEEZY_WEBHOOK_SECRET;
    if (!secret) {
      console.error('LEMONSQUEEZY_WEBHOOK_SECRET not configured');
      return errorResponse('Server configuration error', 500);
    }
    
    if (!verifyWebhookSignature(rawBody, signature, secret)) {
      console.error('Invalid webhook signature');
      return errorResponse('Invalid signature', 401);
    }
    
    const payload: WebhookPayload = JSON.parse(rawBody);
    const eventName = payload.meta.event_name;
    const eventId = `${eventName}-${payload.data.id}`;
    
    // Idempotency check
    if (await db.isEventProcessed(eventId)) {
      console.log('Event already processed:', eventId);
      return successResponse({ message: 'Already processed' });
    }
    
    console.log('Processing webhook:', eventName, payload.data.id);
    
    // Handle different event types
    switch (eventName) {
      case 'order_created':
        await handleOrderCreated(payload);
        break;
        
      case 'subscription_created':
        await handleSubscriptionCreated(payload);
        break;
        
      case 'subscription_updated':
      case 'subscription_payment_success':
        await handleSubscriptionUpdated(payload);
        break;
        
      case 'subscription_cancelled':
      case 'subscription_payment_failed':
        await handleSubscriptionCancelled(payload);
        break;
        
      case 'order_refunded':
        await handleOrderRefunded(payload);
        break;
        
      default:
        console.log('Unhandled event type:', eventName);
    }
    
    // Mark event as processed
    await db.markEventProcessed(eventId, eventName);
    
    return successResponse({ message: 'Webhook processed' });
  } catch (error) {
    console.error('Webhook error:', error);
    return errorResponse('Internal error', 500);
  }
}

async function handleOrderCreated(payload: WebhookPayload) {
  const { attributes } = payload.data;
  const item = attributes.first_order_item;
  
  if (!item) {
    console.error('No order item found');
    return;
  }
  
  // Determine tier from product name
  const tier = parseTierFromProduct(item.product_name, item.variant_name);
  
  // Generate license key
  const licenseKey = generateLicenseKey(tier);
  
  // Create license
  await db.createLicense({
    license_key: licenseKey,
    tier,
    email: attributes.user_email,
    customer_id: attributes.customer_id?.toString(),
    order_id: payload.data.id,
    max_activations: 3,
    // Lifetime licenses never expire, subscriptions get expiry set on renewal
  });
  
  // Send email with license key
  await sendLicenseEmail(
    attributes.user_email,
    licenseKey,
    tier,
    item.product_name
  );
  
  console.log('Created license:', licenseKey, 'for', attributes.user_email);
}

async function handleSubscriptionCreated(payload: WebhookPayload) {
  const { attributes } = payload.data;
  
  // Generate license key
  const licenseKey = generateLicenseKey('pro');
  
  // Calculate expiry
  const expiresAt = calculateExpiryDate(attributes.renews_at || null, attributes.ends_at || null);
  
  // Create license
  await db.createLicense({
    license_key: licenseKey,
    tier: 'pro',
    email: attributes.user_email,
    customer_id: attributes.customer_id?.toString(),
    subscription_id: payload.data.id,
    max_activations: 3,
    expires_at: expiresAt || undefined,
  });
  
  // Send email
  await sendLicenseEmail(
    attributes.user_email,
    licenseKey,
    'pro',
    attributes.product_name || 'Chrono Pro'
  );
  
  console.log('Created subscription license:', licenseKey);
}

async function handleSubscriptionUpdated(payload: WebhookPayload) {
  const { attributes } = payload.data;
  
  // Find license by subscription ID
  const license = await db.getLicenseBySubscriptionId(payload.data.id);
  if (!license) {
    console.error('License not found for subscription:', payload.data.id);
    return;
  }
  
  // Update expiry date
  const expiresAt = calculateExpiryDate(attributes.renews_at || null, attributes.ends_at || null);
  await db.updateLicenseExpiry(license.id, expiresAt);
  
  // Reactivate if was expired
  if (license.status === 'expired') {
    await db.updateLicenseStatus(license.id, 'active');
  }
  
  console.log('Updated subscription license:', license.license_key);
}

async function handleSubscriptionCancelled(payload: WebhookPayload) {
  const { attributes } = payload.data;
  
  const license = await db.getLicenseBySubscriptionId(payload.data.id);
  if (!license) {
    console.error('License not found for subscription:', payload.data.id);
    return;
  }
  
  // Set expiry to end of billing period (allow access until then)
  if (attributes.ends_at) {
    const endsAt = new Date(attributes.ends_at);
    await db.updateLicenseExpiry(license.id, endsAt);
    
    // Only expire if already past end date
    if (endsAt < new Date()) {
      await db.updateLicenseStatus(license.id, 'expired');
    }
  } else {
    // If no end date, expire immediately
    await db.updateLicenseStatus(license.id, 'expired');
  }
  
  console.log('Cancelled subscription license:', license.license_key);
}

async function handleOrderRefunded(payload: WebhookPayload) {
  const license = await db.getLicenseByOrderId(payload.data.id);
  if (!license) {
    console.error('License not found for order:', payload.data.id);
    return;
  }
  
  // Revoke license
  await db.updateLicenseStatus(license.id, 'revoked');
  
  // Remove all activations
  await db.removeAllActivations(license.id);
  
  // Notify user
  if (license.email) {
    await sendRefundEmail(license.email, license.license_key);
  }
  
  console.log('Revoked license due to refund:', license.license_key);
}
