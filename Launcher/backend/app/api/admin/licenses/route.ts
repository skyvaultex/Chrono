import { NextRequest } from 'next/server';
import { db } from '@/lib/db';
import { errorResponse, successResponse } from '@/lib/utils';

// Simple admin auth middleware
function isAdmin(request: NextRequest): boolean {
  const token = request.headers.get('x-admin-token');
  return token === process.env.ADMIN_TOKEN;
}

export async function GET(request: NextRequest) {
  if (!isAdmin(request)) {
    return errorResponse('Unauthorized', 401);
  }
  
  try {
    const { searchParams } = new URL(request.url);
    const query = searchParams.get('q') || searchParams.get('query');
    const limit = parseInt(searchParams.get('limit') || '50');
    const offset = parseInt(searchParams.get('offset') || '0');
    
    let licenses;
    
    if (query) {
      licenses = await db.searchLicenses(query, limit);
    } else {
      licenses = await db.getAllLicenses(limit, offset);
    }
    
    return successResponse({
      licenses,
      count: licenses.length,
      limit,
      offset,
    });
  } catch (error) {
    console.error('Admin licenses error:', error);
    return errorResponse('Internal error', 500);
  }
}
