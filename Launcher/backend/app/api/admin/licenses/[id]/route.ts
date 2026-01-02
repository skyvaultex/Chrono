import { NextRequest } from 'next/server';
import { db } from '@/lib/db';
import { errorResponse, successResponse } from '@/lib/utils';

function isAdmin(request: NextRequest): boolean {
  const token = request.headers.get('x-admin-token');
  return token === process.env.ADMIN_TOKEN;
}

// GET /api/admin/licenses/[id]
export async function GET(
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
    
    const activations = await db.getActivations(license.id);
    
    return successResponse({
      license,
      activations,
    });
  } catch (error) {
    console.error('Admin license detail error:', error);
    return errorResponse('Internal error', 500);
  }
}
