import { NextRequest } from 'next/server';
import { getFeatureLimits } from '@/lib/db';
import { errorResponse, successResponse } from '@/lib/utils';

interface ActivateRequest {
  license_key: string;
  device_id: string;
  device_name?: string;
}

interface LemonSqueezyResponse {
  activated: boolean;
  error: string | null;
  license_key: {
    id: number;
    status: string;
    key: string;
    activation_limit: number;
    activation_usage: number;
    created_at: string;
    expires_at: string | null;
  };
  instance?: {
    id: string;
    name: string;
    created_at: string;
  };
  meta: {
    store_id: number;
    order_id: number;
    product_id: number;
    product_name: string;
    variant_id: number;
    variant_name: string;
    customer_id: number;
    customer_name: string;
    customer_email: string;
  };
}

// Parse tier from LemonSqueezy product/variant name
function parseTierFromProduct(productName: string, variantName?: string): 'pro' | 'lifetime' {
  const name = (variantName || productName).toLowerCase();
  if (name.includes('lifetime')) return 'lifetime';
  return 'pro';
}

export async function POST(request: NextRequest) {
  try {
    const body: ActivateRequest = await request.json();
    
    if (!body.license_key || !body.device_id) {
      return errorResponse('license_key and device_id are required');
    }

    const instanceName = body.device_name || body.device_id;
    const licenseKey = body.license_key.trim();
    
    console.log('Activating license:', licenseKey, 'instance:', instanceName);
    
    // Call LemonSqueezy's License API directly (must use form-urlencoded)
    const response = await fetch('https://api.lemonsqueezy.com/v1/licenses/activate', {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: `license_key=${encodeURIComponent(licenseKey)}&instance_name=${encodeURIComponent(instanceName)}`,
    });
    
    const data = await response.json();
    
    console.log('LemonSqueezy response status:', response.status);
    console.log('LemonSqueezy response:', JSON.stringify(data, null, 2));
    
    // Handle errors from LemonSqueezy
    if (!response.ok || data.activated === false) {
      const errorMessage = data.error || data.message || 'Failed to activate license';
      
      // Provide user-friendly error messages
      if (errorMessage.includes('activation limit')) {
        return errorResponse(
          `Maximum activations reached (${data.license_key?.activation_limit || 'limit'}). Deactivate another device first.`,
          403
        );
      }
      if (errorMessage.includes('not found') || errorMessage.includes('invalid')) {
        return errorResponse('Invalid license key', 404);
      }
      if (data.license_key?.status === 'expired') {
        return errorResponse('License has expired', 403);
      }
      if (data.license_key?.status === 'disabled') {
        return errorResponse('License has been disabled', 403);
      }
      
      return errorResponse(errorMessage, response.status >= 400 ? response.status : 400);
    }
    
    // Determine tier from product name
    const tier = parseTierFromProduct(data.meta.product_name, data.meta.variant_name);
    
    return successResponse({
      success: true,
      tier: tier,
      limits: getFeatureLimits(tier),
      activation: {
        count: data.license_key.activation_usage,
        max: data.license_key.activation_limit,
      },
      instance_id: data.instance?.id,
    });
  } catch (error) {
    console.error('Activate error:', error);
    return errorResponse('Internal error', 500);
  }
}
