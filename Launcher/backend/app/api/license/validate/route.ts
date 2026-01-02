import { NextRequest } from 'next/server';
import { db, getFeatureLimits } from '@/lib/db';
import { isLicenseValid, errorResponse, successResponse } from '@/lib/utils';

interface ValidateRequest {
  license_key: string;
  device_id?: string;
}

export async function POST(request: NextRequest) {
  try {
    const body: ValidateRequest = await request.json();
    
    if (!body.license_key) {
      return errorResponse('license_key is required');
    }
    
    // Find license
    const license = await db.getLicenseByKey(body.license_key);
    
    if (!license) {
      return successResponse({
        valid: false,
        error: 'License not found',
      });
    }
    
    // Check validity
    const valid = isLicenseValid(license);
    
    if (!valid) {
      return successResponse({
        valid: false,
        error: license.status === 'revoked' ? 'License has been revoked' : 'License has expired',
        tier: 'free',
        limits: getFeatureLimits('free'),
      });
    }
    
    // Get activation count
    const activationCount = await db.getActivationCount(license.id);
    
    // Check if device is already activated
    let isActivated = false;
    if (body.device_id) {
      const activations = await db.getActivations(license.id);
      isActivated = activations.some(a => a.device_id === body.device_id);
    }
    
    return successResponse({
      valid: true,
      tier: license.tier,
      status: license.status,
      expires_at: license.expires_at,
      limits: getFeatureLimits(license.tier),
      activation: {
        count: activationCount,
        max: license.max_activations,
        is_activated: isActivated,
        can_activate: isActivated || activationCount < license.max_activations,
      },
    });
  } catch (error) {
    console.error('Validate error:', error);
    return errorResponse('Internal error', 500);
  }
}
