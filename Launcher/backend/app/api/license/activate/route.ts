import { NextRequest } from 'next/server';
import { db, getFeatureLimits } from '@/lib/db';
import { isLicenseValid, errorResponse, successResponse } from '@/lib/utils';

interface ActivateRequest {
  license_key: string;
  device_id: string;
  device_name?: string;
}

export async function POST(request: NextRequest) {
  try {
    const body: ActivateRequest = await request.json();
    
    if (!body.license_key || !body.device_id) {
      return errorResponse('license_key and device_id are required');
    }
    
    // Find license
    const license = await db.getLicenseByKey(body.license_key);
    
    if (!license) {
      return errorResponse('License not found', 404);
    }
    
    // Check validity
    if (!isLicenseValid(license)) {
      return errorResponse(
        license.status === 'revoked' ? 'License has been revoked' : 'License has expired',
        403
      );
    }
    
    // Check existing activations
    const activations = await db.getActivations(license.id);
    const existingActivation = activations.find(a => a.device_id === body.device_id);
    
    // If already activated on this device, just return success
    if (existingActivation) {
      return successResponse({
        success: true,
        tier: license.tier,
        limits: getFeatureLimits(license.tier),
        activation: {
          count: activations.length,
          max: license.max_activations,
        },
      });
    }
    
    // Check if can add new activation
    if (activations.length >= license.max_activations) {
      return errorResponse(
        `Maximum activations reached (${license.max_activations}). Deactivate another device first.`,
        403
      );
    }
    
    // Add activation
    await db.addActivation(license.id, body.device_id, body.device_name);
    
    return successResponse({
      success: true,
      tier: license.tier,
      limits: getFeatureLimits(license.tier),
      activation: {
        count: activations.length + 1,
        max: license.max_activations,
      },
    });
  } catch (error) {
    console.error('Activate error:', error);
    return errorResponse('Internal error', 500);
  }
}
