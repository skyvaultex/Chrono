import { NextRequest } from 'next/server';
import { db } from '@/lib/db';
import { errorResponse, successResponse } from '@/lib/utils';

interface DeactivateRequest {
  license_key: string;
  device_id: string;
}

export async function POST(request: NextRequest) {
  try {
    const body: DeactivateRequest = await request.json();
    
    if (!body.license_key || !body.device_id) {
      return errorResponse('license_key and device_id are required');
    }
    
    // Find license
    const license = await db.getLicenseByKey(body.license_key);
    
    if (!license) {
      return errorResponse('License not found', 404);
    }
    
    // Remove activation
    await db.removeActivation(license.id, body.device_id);
    
    // Get updated count
    const activationCount = await db.getActivationCount(license.id);
    
    return successResponse({
      success: true,
      message: 'Device deactivated',
      activation: {
        count: activationCount,
        max: license.max_activations,
      },
    });
  } catch (error) {
    console.error('Deactivate error:', error);
    return errorResponse('Internal error', 500);
  }
}
