import { Resend } from 'resend';

// Lazy initialization to avoid build-time errors
let resend: Resend | null = null;

function getResend(): Resend {
  if (!resend) {
    if (!process.env.RESEND_API_KEY) {
      throw new Error('RESEND_API_KEY environment variable is not set');
    }
    resend = new Resend(process.env.RESEND_API_KEY);
  }
  return resend;
}

export async function sendLicenseEmail(
  email: string,
  licenseKey: string,
  tier: 'pro' | 'lifetime',
  productName: string
) {
  const tierLabel = tier === 'lifetime' ? 'Lifetime' : 'Pro';
  
  await getResend().emails.send({
    from: process.env.FROM_EMAIL || 'Chrono <hello@chrono.app>',
    to: email,
    subject: `Your Chrono ${tierLabel} License Key`,
    html: `
      <!DOCTYPE html>
      <html>
      <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
      </head>
      <body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px;">
        <div style="text-align: center; margin-bottom: 30px;">
          <h1 style="color: #4F46E5; margin: 0;">⏱️ Chrono</h1>
        </div>
        
        <h2 style="color: #111;">Welcome to Chrono ${tierLabel}!</h2>
        
        <p>Thank you for your purchase of <strong>${productName}</strong>.</p>
        
        <p>Your license key:</p>
        
        <div style="background: #F3F4F6; border-radius: 8px; padding: 16px; text-align: center; margin: 20px 0;">
          <code style="font-size: 20px; font-weight: bold; color: #4F46E5; letter-spacing: 1px;">
            ${licenseKey}
          </code>
        </div>
        
        <p><strong>To activate:</strong></p>
        <ol>
          <li>Open Chrono</li>
          <li>Go to <strong>Tools → Settings</strong></li>
          <li>Paste your license key and click <strong>Activate</strong></li>
        </ol>
        
        <p style="color: #666; font-size: 14px;">
          Keep this email safe. You can use this key on up to 3 devices.
        </p>
        
        <hr style="border: none; border-top: 1px solid #E5E7EB; margin: 30px 0;">
        
        <p style="color: #999; font-size: 12px; text-align: center;">
          Questions? Reply to this email or visit <a href="${process.env.APP_URL}" style="color: #4F46E5;">chrono.app</a>
        </p>
      </body>
      </html>
    `,
  });
}

export async function sendRefundEmail(email: string, licenseKey: string) {
  await getResend().emails.send({
    from: process.env.FROM_EMAIL || 'Chrono <hello@chrono.app>',
    to: email,
    subject: 'Chrono License Deactivated',
    html: `
      <!DOCTYPE html>
      <html>
      <head>
        <meta charset="utf-8">
      </head>
      <body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px;">
        <h2>License Deactivated</h2>
        
        <p>Your Chrono license <code>${licenseKey}</code> has been deactivated due to a refund.</p>
        
        <p>The app will continue to work in Free mode with limited features.</p>
        
        <p>If you believe this is an error, please reply to this email.</p>
        
        <p style="color: #999; font-size: 12px;">— The Chrono Team</p>
      </body>
      </html>
    `,
  });
}

export async function sendExpirationWarningEmail(email: string, daysLeft: number) {
  await getResend().emails.send({
    from: process.env.FROM_EMAIL || 'Chrono <hello@chrono.app>',
    to: email,
    subject: `Your Chrono subscription expires in ${daysLeft} days`,
    html: `
      <!DOCTYPE html>
      <html>
      <head>
        <meta charset="utf-8">
      </head>
      <body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px;">
        <h2>Subscription Expiring Soon</h2>
        
        <p>Your Chrono Pro subscription will expire in <strong>${daysLeft} days</strong>.</p>
        
        <p>To keep all Pro features, please update your payment method or renew your subscription.</p>
        
        <p>
          <a href="${process.env.APP_URL}/account" style="display: inline-block; background: #4F46E5; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px;">
            Manage Subscription
          </a>
        </p>
        
        <p style="color: #999; font-size: 12px;">— The Chrono Team</p>
      </body>
      </html>
    `,
  });
}
