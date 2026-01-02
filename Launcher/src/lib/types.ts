// Types for Tauri commands

export enum GoalType {
  Debt = 'Debt',
  Purchase = 'Purchase',
  Savings = 'Savings'
}

export enum PayType {
  None = 'None',
  Hourly = 'Hourly',
  Fixed = 'Fixed'
}

export interface SessionType {
  id: number;
  name: string;
  color: string;
  hourly_rate?: number;
}

export interface NewSessionType {
  name: string;
  color: string;
  hourly_rate?: number;
}

export interface WorkSession {
  id: number;
  session_type_id: number;
  session_type_name?: string;
  date: string;
  project_name: string;
  hours: number;
  description?: string;
  pay_type?: PayType;
  hourly_rate?: number;
  fixed_amount?: number;
}

export interface NewSession {
  session_type_id: number;
  date: string;
  project_name: string;
  hours: number;
  description?: string;
  pay_type?: PayType;
  hourly_rate?: number;
  fixed_amount?: number;
}

export interface FinancialGoal {
  id: number;
  goal_type: GoalType;
  name: string;
  target_amount: number;
  current_amount: number;
  created_date: string;
  target_date?: string;
}

export interface NewGoal {
  goal_type: GoalType;
  name: string;
  target_amount: number;
  current_amount: number;
  created_date: string;
  target_date?: string;
}

export interface TodaySummary {
  date: string;
  total_hours: number;
  total_pay: number;
  session_hours: Record<string, number>;
}

export interface PaySummary {
  today: number;
  this_month: number;
  this_year: number;
  all_time: number;
}

// ========== ANALYTICS TYPES ==========

export interface AnalyticsSummary {
  total_hours: number;
  total_sessions: number;
  avg_session_length: number;
  longest_session: number;
  total_pay: number;
}

export interface DailyHours {
  date: string;
  hours: number;
  pay: number;
}

export interface CategoryBreakdown {
  category: string;
  color: string;
  hours: number;
  sessions: number;
  pay: number;
}

export interface WeekdayBreakdown {
  weekday: string;
  hours: number;
  sessions: number;
}

export interface AnalyticsData {
  summary: AnalyticsSummary;
  daily_hours: DailyHours[];
  category_breakdown: CategoryBreakdown[];
  weekday_breakdown: WeekdayBreakdown[];
}

// ========== ACHIEVEMENT TYPES ==========

export enum AchievementCategory {
  Presence = 'Presence',
  Awareness = 'Awareness',
  Balance = 'Balance',
  Commitment = 'Commitment',
  Financial = 'Financial'
}

export interface Achievement {
  id: string;
  name: string;
  description: string;
  category: AchievementCategory;
  icon: string;
  unlocked: boolean;
  unlocked_at: string | null;
}

// ========== FINANCIAL SIMULATOR TYPES ==========

export interface GoalProjection {
  goal_id: number;
  goal_name: string;
  remaining: number;
  weeks_to_complete: number | null;
  completion_date: string | null;
}

export interface SimulationResult {
  weekly_income: number;
  weekly_savings: number;
  monthly_income: number;
  monthly_savings: number;
  yearly_income: number;
  yearly_savings: number;
  goal_projections: GoalProjection[];
  sustainability_score: number;
  insights: string[];
}

// ========== FOCUS & BURNOUT TYPES ==========

export interface FocusMetrics {
  focus_score: number;           // 0-100
  fragmentation_score: number;   // 0-100 (higher = more fragmented)
  avg_session_length: number;
  session_count: number;
  total_hours: number;
  longest_streak_days: number;
  current_streak_days: number;
  // Extended fields computed on frontend
  total_sessions?: number;
  longest_session?: number;
  sessions_per_day?: number;
  deep_work_percentage?: number;
  most_productive_hour?: string;
  consistency_score?: number;
}

export interface BurnoutFactor {
  name: string;
  severity: string;
  value: string;
  threshold: string;
}

export interface BurnoutRisk {
  risk_level: string;           // "Low", "Moderate", "High", "Critical"
  risk_score: number;           // 0-100
  factors: BurnoutFactor[];
  recommendations: string[];
}

// ========== INVOICE TYPES ==========

export enum InvoiceStatus {
  Draft = 'Draft',
  Sent = 'Sent',
  Paid = 'Paid',
  Overdue = 'Overdue'
}

export interface InvoiceItem {
  id: number;
  invoice_id: number;
  session_id: number | null;
  description: string;
  hours: number;
  rate: number;
  amount: number;
}

export interface Invoice {
  id: number;
  invoice_number: string;
  client_name: string;
  client_email: string | null;
  created_date: string;
  due_date: string | null;
  status: string;              // "Draft", "Sent", "Paid", "Overdue"
  subtotal: number;
  tax_rate: number | null;
  tax_amount: number;
  total: number;
  notes: string | null;
  items: InvoiceItem[];
}

export interface NewInvoice {
  client_name: string;
  client_email?: string;
  due_date: string;
  tax_rate?: number;
  notes?: string;
  session_ids: number[];
}

// ========== AUTO-TRACKING TYPES ==========

export interface ActivitySuggestion {
  id: number;
  date: string;
  app_name: string;
  window_title: string;
  suggested_project: string | null;
  suggested_session_type_id: number | null;
  duration_minutes: number;
  start_time: string;
  end_time: string;
  status: string;
}

export interface TrackingRule {
  id: number;
  app_pattern: string;
  title_pattern: string | null;
  project_name: string;
  session_type_id: number;
  is_active: boolean;
}

// ========== HABIT TYPES ==========

export interface Habit {
  id: number;
  name: string;
  description: string | null;
  trigger_type: string;       // "after_session", "after_hours", "daily"
  trigger_value: number;      // hours threshold or session count
  reward_description: string;
  is_active: boolean;
  // Computed fields from backend
  current_streak?: number;
  best_streak?: number;
  total_completions?: number;
  trigger_minutes?: number | null;
  target_streak?: number;
}

export interface NewHabit {
  name: string;
  description?: string;
  trigger_type: string;
  trigger_minutes?: number;
  target_streak?: number;
}

export interface HabitLog {
  id: number;
  habit_id: number;
  completed_at: string;
  notes: string | null;
}

// ========== AI ADVISOR TYPES ==========

export interface FinancialInsight {
  category: string;
  severity: string;
  title: string;
  message: string;
  action: string | null;
}

export interface FinancialAnalysis {
  avg_weekly_hours: number;
  avg_weekly_income: number;
  projected_monthly_income: number;
  projected_yearly_income: number;
  income_trend: string;
  insights: FinancialInsight[];
  recommendations: string[];
}

// ========== LICENSE & TIER TYPES ==========

export enum Tier {
  Free = 'Free',
  Pro = 'Pro',
  Lifetime = 'Lifetime'
}

export interface License {
  tier: Tier;
  license_key?: string;
  activated_at?: string;
  expires_at?: string;
}

export interface FeatureLimits {
  max_session_types?: number;  // undefined = unlimited
  max_goals?: number;          // undefined = unlimited
  analytics_days?: number;     // undefined = unlimited
  has_invoices: boolean;
  has_ai_advisor: boolean;
  has_voice_input: boolean;
  has_simulator: boolean;
  has_pdf_export: boolean;
}

export interface CurrentUsage {
  session_type_count: number;
  goal_count: number;
}

export interface LimitCheck {
  allowed: boolean;
  current: number;
  limit?: number;
  feature: string;
}
