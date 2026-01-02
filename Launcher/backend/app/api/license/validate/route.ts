import { NextRequest } from 'next/server';
import { getFeatureLimits } from '@/lib/db';
import { errorResponse, successResponse } from '@/lib/utils';

interface ValidateRequest {
  license_key: string;
  instance_id?: string;
}

interface LemonSqueezyResponse {
  valid: boolean;
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
  instance: {
    id: string;
    name: string;
    created_at: string;
  } | null;
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
    const body: ValidateRequest = await request.json();
    
    if (!body.license_key) {
      return errorResponse('license_key is required');
    }
    
    // Call LemonSqueezy's License API directly
    const params: Record<string, string> = {
      license_key: body.license_key,
    };
    if (body.instance_id) {
      params.instance_id = body.instance_id;
    }
    
    const response = await fetch('https://api.lemonsqueezy.com/v1/licenses/validate', {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: new URLSearchParams(params),
    });
    
    const data: LemonSqueezyResponse = await response.json();
    
    // Not valid
    if (!data.valid) {
      return successResponse({
        valid: false,
        error: data.error || 'Invalid license key',
        tier: 'free',
        limits: getFeatureLimits('free'),
      });
    }
    
    // License expired or disabled
    if (data.license_key.status === 'expired') {
      return successResponse({
        valid: false,
        error: 'License has expired',
        tier: 'free',
        limits: getFeatureLimits('free'),
      });
    }
    
    if (data.license_key.status === 'disabled') {
      return successResponse({
        valid: false,
        error: 'License has been disabled',
        tier: 'free',
        limits: getFeatureLimits('free'),
      });
    }
    
    // Determine tier from product name
    const tier = parseTierFromProduct(data.meta.product_name, data.meta.variant_name);
    
    return successResponse({
      valid: true,
      tier: tier,
      status: data.license_key.status,
      expires_at: data.license_key.expires_at,
      limits: getFeatureLimits(tier),
      activation: {
        count: data.license_key.activation_usage,
        max: data.license_key.activation_limit,
        is_activated: data.instance !== null,
        can_activate: data.license_key.activation_usage < data.license_key.activation_limit,
      },
    });
  } catch (error) {
    console.error('Validate error:', error);
    return errorResponse('Internal error', 500);
  }
}
