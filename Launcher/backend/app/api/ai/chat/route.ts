import { NextRequest } from 'next/server';
import { db, getFeatureLimits } from '@/lib/db';
import { isLicenseValid, errorResponse, successResponse } from '@/lib/utils';
import OpenAI from 'openai';

// Rate limiting - in production, use Redis/Upstash
const rateLimitMap = new Map<string, { count: number; resetAt: number }>();

const FREE_DAILY_LIMIT = 10;
const PRO_DAILY_LIMIT = 100; // Higher limit for pro users

interface ChatRequest {
  license_key: string;
  device_id: string;
  question: string;
  context: AIContext;
}

interface AIContext {
  today_hours: number;
  today_pay: number;
  today_sessions: number;
  period_total_hours: number;
  period_total_sessions: number;
  period_total_pay: number;
  period_avg_session: number;
  period_longest_session: number;
  avg_weekly_income: number;
  avg_daily_hours: number;
  consistency_score: number;
  categories_summary: string;
  best_weekday: string;
  worst_weekday: string;
  weekday_summary: string;
  goals_count: number;
  goals_summary: string;
  total_debt: number;
  total_savings_target: number;
  recent_sessions_summary: string;
}

function getRateLimitKey(licenseKey: string, deviceId: string): string {
  return `${licenseKey}:${deviceId}`;
}

function checkRateLimit(key: string, limit: number): { allowed: boolean; remaining: number; resetAt: number } {
  const now = Date.now();
  const resetTime = new Date().setHours(24, 0, 0, 0); // Midnight tonight
  
  let record = rateLimitMap.get(key);
  
  // Reset if past reset time
  if (!record || now > record.resetAt) {
    record = { count: 0, resetAt: resetTime };
  }
  
  const remaining = Math.max(0, limit - record.count);
  
  if (record.count >= limit) {
    return { allowed: false, remaining: 0, resetAt: record.resetAt };
  }
  
  // Increment count
  record.count++;
  rateLimitMap.set(key, record);
  
  return { allowed: true, remaining: remaining - 1, resetAt: record.resetAt };
}

export async function POST(request: NextRequest) {
  try {
    const body: ChatRequest = await request.json();
    
    // Validate required fields
    if (!body.license_key || !body.device_id || !body.question || !body.context) {
      return errorResponse('Missing required fields: license_key, device_id, question, context');
    }
    
    // Validate license
    const license = await db.getLicenseByKey(body.license_key);
    
    if (!license || !isLicenseValid(license)) {
      return errorResponse('Invalid or expired license', 403);
    }
    
    // Check if device is activated
    const activations = await db.getActivations(license.id);
    const isActivated = activations.some(a => a.device_id === body.device_id);
    
    if (!isActivated) {
      return errorResponse('Device not activated for this license', 403);
    }
    
    // Rate limiting - Free: 10/day, Pro: 100/day
    const dailyLimit = license.tier === 'free' ? FREE_DAILY_LIMIT : PRO_DAILY_LIMIT;
    const rateLimitKey = getRateLimitKey(body.license_key, body.device_id);
    const rateLimit = checkRateLimit(rateLimitKey, dailyLimit);
    
    if (!rateLimit.allowed) {
      return Response.json({
        error: `Daily limit reached (${dailyLimit} queries/day). Resets at midnight.`,
        remaining: rateLimit.remaining,
        reset_at: new Date(rateLimit.resetAt).toISOString(),
      }, { status: 429 });
    }
    
    // Build system prompt
    const systemPrompt = buildSystemPrompt(body.context);
    
    // Call OpenAI
    const openai = new OpenAI({
      apiKey: process.env.OPENAI_API_KEY,
    });
    
    const completion = await openai.chat.completions.create({
      model: 'gpt-4o-mini',
      messages: [
        { role: 'system', content: systemPrompt },
        { role: 'user', content: body.question },
      ],
      max_tokens: 600,
      temperature: 0.7,
    });
    
    const response = completion.choices[0]?.message?.content || 'Sorry, I couldn\'t generate a response.';
    
    return successResponse({
      response,
      usage: {
        remaining: rateLimit.remaining,
        limit: dailyLimit,
        reset_at: new Date(rateLimit.resetAt).toISOString(),
      },
    });
    
  } catch (error) {
    console.error('AI chat error:', error);
    
    if (error instanceof OpenAI.APIError) {
      return errorResponse(`AI service error: ${error.message}`, 502);
    }
    
    return errorResponse('Internal error', 500);
  }
}

function buildSystemPrompt(ctx: AIContext): string {
  return `You are a helpful financial and productivity advisor for Chrono, a work tracking app.
The user tracks work sessions across different categories and manages financial goals.

═══════════════════════════════════════════
TODAY'S SNAPSHOT
═══════════════════════════════════════════
• Hours worked: ${ctx.today_hours.toFixed(1)}h
• Sessions completed: ${ctx.today_sessions}
• Pay earned: $${ctx.today_pay.toFixed(2)}

═══════════════════════════════════════════
LAST 30 DAYS ANALYTICS
═══════════════════════════════════════════
• Total hours: ${ctx.period_total_hours.toFixed(1)}h
• Total sessions: ${ctx.period_total_sessions}
• Total pay: $${ctx.period_total_pay.toFixed(2)}
• Average session length: ${ctx.period_avg_session.toFixed(1)}h
• Longest session: ${ctx.period_longest_session.toFixed(1)}h
• Average daily hours: ${ctx.avg_daily_hours.toFixed(1)}h
• Consistency score: ${ctx.consistency_score}% (days with logged work)

═══════════════════════════════════════════
WORK PATTERNS BY CATEGORY
═══════════════════════════════════════════
${ctx.categories_summary}

═══════════════════════════════════════════
WEEKDAY PATTERNS
═══════════════════════════════════════════
• Best day: ${ctx.best_weekday}
• Lightest day: ${ctx.worst_weekday}
${ctx.weekday_summary}

═══════════════════════════════════════════
INCOME
═══════════════════════════════════════════
• Average weekly income: $${ctx.avg_weekly_income.toFixed(2)}

═══════════════════════════════════════════
FINANCIAL GOALS (${ctx.goals_count} active)
═══════════════════════════════════════════
${ctx.goals_summary}
• Total debt being paid: $${ctx.total_debt.toFixed(2)}
• Total savings target: $${ctx.total_savings_target.toFixed(2)}

═══════════════════════════════════════════
RECENT ACTIVITY
═══════════════════════════════════════════
${ctx.recent_sessions_summary}

ADVISOR GUIDELINES:
- Be concise, practical, and encouraging
- Reference specific data from above when relevant
- Give actionable advice, not generic tips
- Identify patterns (good and concerning)
- Keep responses under 250 words unless detailed analysis requested
- Don't repeat all the stats back - the user can see them
- Focus on insights, trends, and recommendations`;
}
