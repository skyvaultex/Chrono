import { invoke } from '@tauri-apps/api/tauri';
import type {
  WorkSession,
  NewSession,
  FinancialGoal,
  NewGoal,
  TodaySummary,
  SessionType,
  NewSessionType,
  PaySummary,
  GoalType,
  AnalyticsData,
  Achievement,
  SimulationResult,
  FocusMetrics,
  BurnoutRisk,
  Invoice,
  NewInvoice,
  Habit,
  NewHabit,
  HabitLog,
  ActivitySuggestion,
  TrackingRule,
  FinancialAnalysis
} from './types';

// ========== SESSION TYPE API ==========

export async function getAllSessionTypes(): Promise<SessionType[]> {
  return await invoke('get_all_session_types');
}

export async function addSessionType(sessionType: NewSessionType): Promise<number> {
  return await invoke('add_session_type', { sessionType });
}

export async function updateSessionType(sessionType: SessionType): Promise<void> {
  return await invoke('update_session_type', { sessionType });
}

export async function deleteSessionType(id: number): Promise<void> {
  return await invoke('delete_session_type', { id });
}

// ========== SESSION API ==========

export async function getAllSessions(): Promise<WorkSession[]> {
  return await invoke('get_all_sessions');
}

export async function getSessionsByDate(date: string): Promise<WorkSession[]> {
  return await invoke('get_sessions_by_date', { date });
}

export async function getSessionsByTypeId(sessionTypeId: number): Promise<WorkSession[]> {
  return await invoke('get_sessions_by_type_id', { sessionTypeId });
}

export async function addSession(session: NewSession): Promise<number> {
  return await invoke('add_session', { session });
}

export async function updateSession(session: WorkSession): Promise<void> {
  return await invoke('update_session', { session });
}

export async function deleteSession(id: number): Promise<void> {
  return await invoke('delete_session', { id });
}

// ========== GOAL API ==========

export async function getAllGoals(): Promise<FinancialGoal[]> {
  return await invoke('get_all_goals');
}

export async function addGoal(goal: NewGoal): Promise<number> {
  return await invoke('add_goal', { goal });
}

export async function updateGoal(goal: FinancialGoal): Promise<void> {
  return await invoke('update_goal', { goal });
}

export async function addContribution(goalId: number, amount: number): Promise<void> {
  return await invoke('add_contribution', { goalId, amount });
}

export async function deleteGoal(id: number): Promise<void> {
  return await invoke('delete_goal', { id });
}

// ========== PROJECT API ==========

export async function getProjectsByTypeId(sessionTypeId: number): Promise<string[]> {
  return await invoke('get_projects_by_type_id', { sessionTypeId });
}

// ========== DASHBOARD API ==========

export async function getTodaySummary(): Promise<TodaySummary> {
  return await invoke('get_today_summary');
}

export async function getPaySummary(): Promise<PaySummary> {
  return await invoke('get_pay_summary');
}

export async function getRecentSessions(limit: number): Promise<WorkSession[]> {
  return await invoke('get_recent_sessions', { limit });
}

export async function calculateAvgWeeklyIncome(): Promise<number> {
  return await invoke('calculate_avg_weekly_income');
}

export async function getGoalEta(goalId: number): Promise<string | null> {
  return await invoke('get_goal_eta', { goalId });
}

export async function getCurrentDate(): Promise<string> {
  return await invoke('get_current_date');
}

// ========== ANALYTICS API ==========

export async function getAnalytics(rangeStart: string, rangeEnd: string): Promise<AnalyticsData> {
  return await invoke('get_analytics', { rangeStart, rangeEnd });
}

// ========== AI ADVISOR API ==========

export interface AIContext {
  // Today's snapshot
  today_hours: number;
  today_pay: number;
  today_sessions: number;
  
  // Period analytics (last 30 days)
  period_total_hours: number;
  period_total_sessions: number;
  period_total_pay: number;
  period_avg_session: number;
  period_longest_session: number;
  
  // Averages
  avg_weekly_income: number;
  avg_daily_hours: number;
  consistency_score: number;
  
  // Category breakdown
  categories_summary: string;
  
  // Weekday patterns
  best_weekday: string;
  worst_weekday: string;
  weekday_summary: string;
  
  // Goals
  goals_count: number;
  goals_summary: string;
  total_debt: number;
  total_savings_target: number;
  
  // Recent activity
  recent_sessions_summary: string;
}

export interface AIAdvisorResponse {
  content: string;
  remaining_queries: number;
  daily_limit: number;
}

export async function askAiAdvisor(question: string, context: AIContext): Promise<AIAdvisorResponse> {
  return await invoke('ask_ai_advisor', { question, context });
}

// ========== ACHIEVEMENT API ==========

export async function getAchievements(): Promise<Achievement[]> {
  return await invoke('get_achievements');
}

export async function checkAndUnlockAchievements(): Promise<string[]> {
  return await invoke('check_and_unlock_achievements');
}

export async function logAppEvent(eventType: string, eventData?: string): Promise<void> {
  return await invoke('log_app_event', { eventType, eventData });
}

// ========== FINANCIAL SIMULATOR API ==========

export async function simulateFinancialScenario(
  hoursPerWeek: number,
  hourlyRate: number,
  weeklyExpenses: number
): Promise<SimulationResult> {
  return await invoke('simulate_financial_scenario', { hoursPerWeek, hourlyRate, weeklyExpenses });
}

export async function getCurrentFinancialBaseline(): Promise<[number, number]> {
  return await invoke('get_current_financial_baseline');
}

// ========== FOCUS & BURNOUT API ==========

export async function getFocusMetrics(days: number): Promise<FocusMetrics> {
  return await invoke('get_focus_metrics', { days });
}

export async function getBurnoutRisk(days: number): Promise<BurnoutRisk> {
  return await invoke('get_burnout_risk', { days });
}

// ========== INVOICE API ==========

export async function getAllInvoices(): Promise<Invoice[]> {
  return await invoke('get_all_invoices');
}

export async function getInvoice(id: number): Promise<Invoice> {
  return await invoke('get_invoice', { id });
}

export async function createInvoice(invoice: NewInvoice): Promise<number> {
  return await invoke('create_invoice', { invoice });
}

export async function updateInvoiceStatus(id: number, status: string): Promise<void> {
  return await invoke('update_invoice_status', { id, status });
}

export async function deleteInvoice(id: number): Promise<void> {
  return await invoke('delete_invoice', { id });
}

export async function getUninvoicedSessions(): Promise<WorkSession[]> {
  return await invoke('get_uninvoiced_sessions');
}

// ========== HABIT API ==========

export async function getAllHabits(): Promise<Habit[]> {
  return await invoke('get_all_habits');
}

export async function createHabit(habit: NewHabit): Promise<number> {
  // Convert NewHabit to the format expected by the backend
  // All fields must be explicitly set - Rust expects non-optional values
  const fullHabit = {
    id: 0, // Will be assigned by backend
    name: habit.name,
    description: habit.description || null,
    trigger_type: habit.trigger_type,
    trigger_value: habit.trigger_minutes ? habit.trigger_minutes / 60.0 : 1.0,
    reward_description: habit.description || 'Complete the habit',
    is_active: true,
    target_streak: habit.target_streak ?? 7,
    trigger_minutes: habit.trigger_minutes ?? null,
    // Required computed fields - initialize to 0
    current_streak: 0,
    best_streak: 0,
    total_completions: 0,
  };
  console.log('Creating habit with data:', fullHabit);
  return await invoke('add_habit', { habit: fullHabit });
}

export async function addHabit(habit: Habit): Promise<number> {
  return await invoke('add_habit', { habit });
}

export async function updateHabit(habit: Habit): Promise<void> {
  return await invoke('update_habit', { habit });
}

export async function deleteHabit(id: number): Promise<void> {
  return await invoke('delete_habit', { id });
}

export async function logHabit(habitId: number, completed: boolean, notes?: string): Promise<number> {
  return await invoke('log_habit_completion', { habitId, notes });
}

export async function logHabitCompletion(habitId: number, notes?: string): Promise<number> {
  return await invoke('log_habit_completion', { habitId, notes });
}

export async function getPendingHabits(): Promise<Habit[]> {
  return await invoke('get_pending_habits');
}

// ========== AUTO-TRACKING API ==========

export async function getActivitySuggestions(): Promise<ActivitySuggestion[]> {
  return await invoke('get_activity_suggestions');
}

export async function acceptActivitySuggestion(
  suggestionId: number,
  sessionTypeId: number,
  projectName: string
): Promise<number> {
  return await invoke('accept_activity_suggestion', { suggestionId, sessionTypeId, projectName });
}

export async function dismissActivitySuggestion(suggestionId: number): Promise<void> {
  return await invoke('dismiss_activity_suggestion', { suggestionId });
}

export async function getTrackingRules(): Promise<TrackingRule[]> {
  return await invoke('get_tracking_rules');
}

export async function addTrackingRule(rule: TrackingRule): Promise<number> {
  return await invoke('add_tracking_rule', { rule });
}

export async function deleteTrackingRule(id: number): Promise<void> {
  return await invoke('delete_tracking_rule', { id });
}

// ========== ENHANCED AI ADVISOR API ==========

export async function getFinancialAnalysis(): Promise<FinancialAnalysis> {
  return await invoke('get_financial_analysis');
}

// ========== LICENSE API ==========

import type { License, FeatureLimits, CurrentUsage, LimitCheck } from './types';

export async function getLicense(): Promise<License> {
  return await invoke('get_license');
}

export async function getFeatureLimits(): Promise<FeatureLimits> {
  return await invoke('get_feature_limits');
}

export async function getCurrentUsage(): Promise<CurrentUsage> {
  return await invoke('get_current_usage');
}

export async function activateLicense(licenseKey: string): Promise<License> {
  return await invoke('activate_license', { licenseKey });
}

export async function deactivateLicense(): Promise<License> {
  return await invoke('deactivate_license');
}

export async function canCreateSessionType(): Promise<LimitCheck> {
  return await invoke('can_create_session_type');
}

export async function canCreateGoal(): Promise<LimitCheck> {
  return await invoke('can_create_goal');
}
