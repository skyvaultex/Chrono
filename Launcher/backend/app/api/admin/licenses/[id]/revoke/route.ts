import { NextRequest } from 'next/server';
import { db } from '@/lib/db';
import { errorResponse, successResponse } from '@/lib/utils';

function isAdmin(request: NextRequest): boolean {
  const token = request.headers.get('x-admin-token');
  return token === process.env.ADMIN_TOKEN;
}

// POST /api/admin/licenses/[id]/revoke
export async function POST(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  if (!isAdmin(request)) {
    return errorResponse('Unauthorized', 401);
  }
  
  try {
    const license = await db.getLicenseById(params.id);
    
    if (!license) {
      return errorResponse('License not found', 404);
    }
    
    // Revoke the license
    await db.updateLicenseStatus(license.id, 'revoked');
    
    // Remove all activations
    await db.removeAllActivations(license.id);
    
    return successResponse({
      success: true,
      message: 'License revoked',
      license_key: license.license_key,
    });
  } catch (error) {
    console.error('Admin revoke error:', error);
    return errorResponse('Internal error', 500);
  }
}
