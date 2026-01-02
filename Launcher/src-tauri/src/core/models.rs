use serde::{Deserialize, Serialize};

/// User-defined session type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionType {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub hourly_rate: Option<f64>,
}

/// DTO for creating a new session type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSessionType {
    pub name: String,
    pub color: String,
    pub hourly_rate: Option<f64>,
}

/// Pay type for sessions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PayType {
    None,
    Hourly,
    Fixed,
}

impl PayType {
    pub fn to_string(&self) -> String {
        match self {
            PayType::None => "None".to_string(),
            PayType::Hourly => "Hourly".to_string(),
            PayType::Fixed => "Fixed".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "None" | "" => Ok(PayType::None),
            "Hourly" => Ok(PayType::Hourly),
            "Fixed" => Ok(PayType::Fixed),
            _ => Err(format!("Invalid pay type: {}", s)),
        }
    }
}

/// Work session record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSession {
    pub id: i64,
    pub session_type_id: i64,
    pub session_type_name: Option<String>,
    pub date: String,
    pub project_name: String,
    pub hours: f64,
    pub description: Option<String>,
    pub pay_type: Option<PayType>,
    pub hourly_rate: Option<f64>,
    pub fixed_amount: Option<f64>,
}

impl WorkSession {
    /// Calculate pay for this session
    pub fn calculate_pay(&self) -> f64 {
        match self.pay_type {
            Some(PayType::Hourly) => {
                self.hourly_rate.unwrap_or(0.0) * self.hours
            }
            Some(PayType::Fixed) => {
                self.fixed_amount.unwrap_or(0.0)
            }
            _ => 0.0,
        }
    }

    /// Validate session data
    pub fn validate(&self) -> Result<(), String> {
        if self.project_name.trim().is_empty() {
            return Err("Project name cannot be empty".to_string());
        }
        if self.hours < 0.1 || self.hours > 24.0 {
            return Err("Hours must be between 0.1 and 24.0".to_string());
        }
        if self.date.is_empty() {
            return Err("Date cannot be empty".to_string());
        }
        Ok(())
    }
}

/// DTO for creating a new session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSession {
    pub session_type_id: i64,
    pub date: String,
    pub project_name: String,
    pub hours: f64,
    pub description: Option<String>,
    pub pay_type: Option<PayType>,
    pub hourly_rate: Option<f64>,
    pub fixed_amount: Option<f64>,
}

/// Type of financial goal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalType {
    Debt,
    Purchase,
    Savings,
}

impl GoalType {
    pub fn to_string(&self) -> String {
        match self {
            GoalType::Debt => "Debt".to_string(),
            GoalType::Purchase => "Purchase".to_string(),
            GoalType::Savings => "Savings".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "Debt" => Ok(GoalType::Debt),
            "Purchase" => Ok(GoalType::Purchase),
            "Savings" => Ok(GoalType::Savings),
            _ => Err(format!("Invalid goal type: {}", s)),
        }
    }
}

/// Financial goal record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialGoal {
    pub id: i64,
    pub goal_type: GoalType,
    pub name: String,
    pub target_amount: f64,
    pub current_amount: f64,
    pub created_date: String,
    pub target_date: Option<String>,
}

impl FinancialGoal {
    pub fn progress_percent(&self) -> f64 {
        if self.target_amount <= 0.0 {
            return 0.0;
        }
        ((self.current_amount / self.target_amount) * 100.0).min(100.0)
    }

    pub fn remaining_amount(&self) -> f64 {
        (self.target_amount - self.current_amount).max(0.0)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Goal name cannot be empty".to_string());
        }
        if self.target_amount <= 0.0 {
            return Err("Target amount must be positive".to_string());
        }
        if self.current_amount < 0.0 {
            return Err("Current amount cannot be negative".to_string());
        }
        Ok(())
    }
}

/// DTO for creating a new goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGoal {
    pub goal_type: GoalType,
    pub name: String,
    pub target_amount: f64,
    pub current_amount: f64,
    pub created_date: String,
    pub target_date: Option<String>,
}

/// Project cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCache {
    pub id: i64,
    pub project_name: String,
    pub session_type_id: i64,
    pub last_used: String,
}

// ========== ACHIEVEMENTS ==========

/// Achievement category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementCategory {
    Presence,
    Awareness,
    Balance,
    Commitment,
    Financial,
}

/// Achievement definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementDef {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: AchievementCategory,
    pub icon: &'static str,
}

/// Unlocked achievement with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: AchievementCategory,
    pub icon: String,
    pub unlocked: bool,
    pub unlocked_at: Option<String>,
}

/// All achievement definitions (the v1 set - 15 achievements)
pub const ACHIEVEMENTS: &[AchievementDef] = &[
    // 🌱 Presence (4)
    AchievementDef {
        id: "first_step",
        name: "First Step",
        description: "Logged your first session",
        category: AchievementCategory::Presence,
        icon: "🌱",
    },
    AchievementDef {
        id: "back_again",
        name: "Back Again",
        description: "Used ChronoDesk on 3 different days",
        category: AchievementCategory::Presence,
        icon: "👋",
    },
    AchievementDef {
        id: "getting_comfortable",
        name: "Getting Comfortable",
        description: "Logged 10 total sessions",
        category: AchievementCategory::Presence,
        icon: "🪴",
    },
    AchievementDef {
        id: "part_of_routine",
        name: "Part of the Routine",
        description: "Used ChronoDesk across 7 different days",
        category: AchievementCategory::Presence,
        icon: "🌿",
    },
    
    // 🧠 Awareness (4)
    AchievementDef {
        id: "curious_mind",
        name: "Curious Mind",
        description: "Opened Analytics for the first time",
        category: AchievementCategory::Awareness,
        icon: "🧠",
    },
    AchievementDef {
        id: "pattern_noticed",
        name: "Pattern Noticed",
        description: "Viewed Analytics on 5 different days",
        category: AchievementCategory::Awareness,
        icon: "🔍",
    },
    AchievementDef {
        id: "zoomed_out",
        name: "Zoomed Out",
        description: "Explored at least 3 different time ranges in Analytics",
        category: AchievementCategory::Awareness,
        icon: "🔭",
    },
    AchievementDef {
        id: "connecting_dots",
        name: "Connecting the Dots",
        description: "Used Analytics and Advisor in the same day",
        category: AchievementCategory::Awareness,
        icon: "🧩",
    },
    
    // ⚖️ Balance (3)
    AchievementDef {
        id: "paced_yourself",
        name: "Paced Yourself",
        description: "No session longer than 6 hours in a full week",
        category: AchievementCategory::Balance,
        icon: "⚖️",
    },
    AchievementDef {
        id: "sustainable_week",
        name: "Sustainable Week",
        description: "Averaged under 8 hours per day for a full week",
        category: AchievementCategory::Balance,
        icon: "🌊",
    },
    AchievementDef {
        id: "human_weekend",
        name: "Human Weekend",
        description: "Logged less than 3 hours total on a weekend",
        category: AchievementCategory::Balance,
        icon: "☀️",
    },
    
    // 🏗️ Commitment (3)
    AchievementDef {
        id: "long_run",
        name: "In It for the Long Run",
        description: "Logged sessions in 3 different calendar weeks",
        category: AchievementCategory::Commitment,
        icon: "🏃",
    },
    AchievementDef {
        id: "one_full_month",
        name: "One Full Month",
        description: "Tracked time across 4 different weeks",
        category: AchievementCategory::Commitment,
        icon: "📅",
    },
    AchievementDef {
        id: "hundred_hours",
        name: "Hundred Hours",
        description: "Tracked 100 total hours",
        category: AchievementCategory::Commitment,
        icon: "💯",
    },
    
    // 💰 Financial (1)
    AchievementDef {
        id: "first_dollar",
        name: "First Dollar",
        description: "Tracked your first paid session",
        category: AchievementCategory::Financial,
        icon: "💵",
    },
];

// ========== FINANCIAL SIMULATOR ==========

/// Financial scenario for simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialScenario {
    pub hours_per_week: f64,
    pub hourly_rate: f64,
    pub weekly_expenses: f64,
    pub goals: Vec<GoalProjection>,
}

/// Goal projection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalProjection {
    pub goal_id: i64,
    pub goal_name: String,
    pub remaining: f64,
    pub weeks_to_complete: Option<f64>,
    pub completion_date: Option<String>,
}

/// Financial simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub weekly_income: f64,
    pub weekly_savings: f64,
    pub monthly_income: f64,
    pub monthly_savings: f64,
    pub yearly_income: f64,
    pub yearly_savings: f64,
    pub goal_projections: Vec<GoalProjection>,
    pub sustainability_score: f64, // 0-100
    pub insights: Vec<String>,
}

// ========== FOCUS & BURNOUT ==========

/// Focus metrics for a time period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusMetrics {
    pub focus_score: f64,           // 0-100
    pub fragmentation_score: f64,   // 0-100 (lower = more fragmented)
    pub avg_session_length: f64,
    pub session_count: usize,
    pub total_hours: f64,
    pub longest_streak_days: i32,
    pub current_streak_days: i32,
}

/// Burnout risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnoutRisk {
    pub risk_level: String,         // "Low", "Moderate", "High", "Critical"
    pub risk_score: f64,            // 0-100
    pub factors: Vec<BurnoutFactor>,
    pub recommendations: Vec<String>,
}

/// Individual burnout risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnoutFactor {
    pub name: String,
    pub severity: String,           // "ok", "warning", "danger"
    pub value: String,
    pub threshold: String,
}

// ========== INVOICING ==========

/// Invoice status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Paid,
    Overdue,
}

impl InvoiceStatus {
    pub fn to_string(&self) -> String {
        match self {
            InvoiceStatus::Draft => "Draft".to_string(),
            InvoiceStatus::Sent => "Sent".to_string(),
            InvoiceStatus::Paid => "Paid".to_string(),
            InvoiceStatus::Overdue => "Overdue".to_string(),
        }
    }
    
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "Draft" => Ok(InvoiceStatus::Draft),
            "Sent" => Ok(InvoiceStatus::Sent),
            "Paid" => Ok(InvoiceStatus::Paid),
            "Overdue" => Ok(InvoiceStatus::Overdue),
            _ => Err(format!("Invalid invoice status: {}", s)),
        }
    }
}

/// Invoice record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: i64,
    pub invoice_number: String,
    pub client_name: String,
    pub client_email: Option<String>,
    pub created_date: String,
    pub due_date: String,
    pub status: InvoiceStatus,
    pub subtotal: f64,
    pub tax_rate: Option<f64>,
    pub tax_amount: f64,
    pub total: f64,
    pub notes: Option<String>,
    pub items: Vec<InvoiceItem>,
}

/// Invoice line item (linked to session)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub id: i64,
    pub invoice_id: i64,
    pub session_id: Option<i64>,
    pub description: String,
    pub hours: f64,
    pub rate: f64,
    pub amount: f64,
}

/// DTO for creating new invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewInvoice {
    pub client_name: String,
    pub client_email: Option<String>,
    pub due_date: String,
    pub tax_rate: Option<f64>,
    pub notes: Option<String>,
    pub session_ids: Vec<i64>,
}

// ========== AUTO-TRACKING ==========

/// Detected activity suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitySuggestion {
    pub id: i64,
    pub date: String,
    pub app_name: String,
    pub window_title: String,
    pub suggested_project: Option<String>,
    pub suggested_session_type_id: Option<i64>,
    pub duration_minutes: f64,
    pub start_time: String,
    pub end_time: String,
    pub status: String, // "pending", "accepted", "dismissed"
}

/// Activity tracking rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingRule {
    pub id: i64,
    pub app_pattern: String,
    pub title_pattern: Option<String>,
    pub project_name: String,
    pub session_type_id: i64,
    pub is_active: bool,
}

// ========== HABITS ==========

/// Habit definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub trigger_type: String,       // "after_session", "after_hours", "daily"
    pub trigger_value: f64,         // hours threshold or session count
    pub reward_description: String,
    pub is_active: bool,
    // Computed fields
    pub current_streak: i32,
    pub best_streak: i32,
    pub total_completions: i32,
    pub target_streak: i32,
    pub trigger_minutes: Option<i32>,
}

/// Habit completion log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitLog {
    pub id: i64,
    pub habit_id: i64,
    pub completed_at: String,
    pub notes: Option<String>,
}

// ========== AI ADVISOR ==========

/// Financial insight from AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialInsight {
    pub category: String,           // "income", "expenses", "goals", "sustainability"
    pub severity: String,           // "info", "warning", "critical", "success"
    pub title: String,
    pub message: String,
    pub action: Option<String>,
}

/// Comprehensive financial analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialAnalysis {
    pub avg_weekly_hours: f64,
    pub avg_weekly_income: f64,
    pub projected_monthly_income: f64,
    pub projected_yearly_income: f64,
    pub income_trend: String,       // "increasing", "stable", "decreasing"
    pub insights: Vec<FinancialInsight>,
    pub recommendations: Vec<String>,
}

// ========== LICENSE & TIERS ==========

/// User tier (Free, Pro, Lifetime)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    Free,
    Pro,
    Lifetime,
}

impl Tier {
    pub fn to_string(&self) -> String {
        match self {
            Tier::Free => "Free".to_string(),
            Tier::Pro => "Pro".to_string(),
            Tier::Lifetime => "Lifetime".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "Pro" => Tier::Pro,
            "Lifetime" => Tier::Lifetime,
            _ => Tier::Free,
        }
    }
}

/// License information stored locally
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub tier: Tier,
    pub license_key: Option<String>,
    pub activated_at: Option<String>,
    pub expires_at: Option<String>,
}

impl Default for License {
    fn default() -> Self {
        License {
            tier: Tier::Free,
            license_key: None,
            activated_at: None,
            expires_at: None,
        }
    }
}

/// Feature limits based on tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureLimits {
    pub max_session_types: Option<u32>,  // None = unlimited
    pub max_goals: Option<u32>,          // None = unlimited
    pub analytics_days: Option<u32>,     // None = unlimited, Some(7) = 7 days
    pub has_invoices: bool,
    pub has_ai_advisor: bool,
    pub has_voice_input: bool,
    pub has_simulator: bool,
    pub has_pdf_export: bool,
}

impl FeatureLimits {
    pub fn for_tier(tier: Tier) -> Self {
        match tier {
            Tier::Free => FeatureLimits {
                max_session_types: Some(2),
                max_goals: Some(3),
                analytics_days: Some(7),
                has_invoices: false,
                has_ai_advisor: true,  // Free gets 10/day, Pro gets 100/day
                has_voice_input: false,
                has_simulator: false,
                has_pdf_export: false,
            },
            Tier::Pro | Tier::Lifetime => FeatureLimits {
                max_session_types: None,
                max_goals: None,
                analytics_days: None,
                has_invoices: true,
                has_ai_advisor: true,
                has_voice_input: true,
                has_simulator: true,
                has_pdf_export: true,
            },
        }
    }
}
