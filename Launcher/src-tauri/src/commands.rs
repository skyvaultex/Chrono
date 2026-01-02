use tauri::State;
use crate::core::models::*;
use crate::core::logic;
use crate::storage::db::{Database, PaySummary, AnalyticsData};
use std::collections::HashMap;

// ========== SESSION TYPE COMMANDS ==========

#[tauri::command]
pub fn get_all_session_types(db: State<Database>) -> Result<Vec<SessionType>, String> {
    db.get_all_session_types().map_err(|e| format!("Failed to get session types: {}", e))
}

#[tauri::command]
pub fn add_session_type(db: State<Database>, session_type: NewSessionType) -> Result<i64, String> {
    if session_type.name.trim().is_empty() {
        return Err("Session type name cannot be empty".to_string());
    }
    db.add_session_type(session_type).map_err(|e| format!("Failed to add session type: {}", e))
}

#[tauri::command]
pub fn update_session_type(db: State<Database>, session_type: SessionType) -> Result<(), String> {
    db.update_session_type(&session_type).map_err(|e| format!("Failed to update session type: {}", e))
}

#[tauri::command]
pub fn delete_session_type(db: State<Database>, id: i64) -> Result<(), String> {
    db.delete_session_type(id).map_err(|e| format!("Failed to delete session type: {}", e))
}

// ========== SESSION COMMANDS ==========

#[tauri::command]
pub fn get_all_sessions(db: State<Database>) -> Result<Vec<WorkSession>, String> {
    db.get_all_sessions().map_err(|e| format!("Failed to get sessions: {}", e))
}

#[tauri::command]
pub fn get_sessions_by_date(db: State<Database>, date: String) -> Result<Vec<WorkSession>, String> {
    db.get_sessions_by_date(&date).map_err(|e| format!("Failed to get sessions by date: {}", e))
}

#[tauri::command]
pub fn get_sessions_by_type_id(db: State<Database>, session_type_id: i64) -> Result<Vec<WorkSession>, String> {
    db.get_sessions_by_type_id(session_type_id).map_err(|e| format!("Failed to get sessions by type: {}", e))
}

#[tauri::command]
pub fn add_session(db: State<Database>, session: NewSession) -> Result<i64, String> {
    if session.hours < 0.1 || session.hours > 24.0 {
        return Err("Hours must be between 0.1 and 24.0".to_string());
    }
    if session.project_name.trim().is_empty() {
        return Err("Project name cannot be empty".to_string());
    }
    db.add_session(session).map_err(|e| format!("Failed to add session: {}", e))
}

#[tauri::command]
pub fn update_session(db: State<Database>, session: WorkSession) -> Result<(), String> {
    session.validate()?;
    db.update_session(&session).map_err(|e| format!("Failed to update session: {}", e))
}

#[tauri::command]
pub fn delete_session(db: State<Database>, id: i64) -> Result<(), String> {
    db.delete_session(id).map_err(|e| format!("Failed to delete session: {}", e))
}

// ========== GOAL COMMANDS ==========

#[tauri::command]
pub fn get_all_goals(db: State<Database>) -> Result<Vec<FinancialGoal>, String> {
    db.get_all_goals().map_err(|e| format!("Failed to get goals: {}", e))
}

#[tauri::command]
pub fn add_goal(db: State<Database>, goal: NewGoal) -> Result<i64, String> {
    if goal.target_amount <= 0.0 {
        return Err("Target amount must be positive".to_string());
    }
    if goal.name.trim().is_empty() {
        return Err("Goal name cannot be empty".to_string());
    }
    db.add_goal(goal).map_err(|e| format!("Failed to add goal: {}", e))
}

#[tauri::command]
pub fn update_goal(db: State<Database>, goal: FinancialGoal) -> Result<(), String> {
    goal.validate()?;
    db.update_goal(&goal).map_err(|e| format!("Failed to update goal: {}", e))
}

#[tauri::command]
pub fn add_contribution(db: State<Database>, goal_id: i64, amount: f64) -> Result<(), String> {
    if amount <= 0.0 {
        return Err("Contribution amount must be positive".to_string());
    }
    db.add_contribution(goal_id, amount).map_err(|e| format!("Failed to add contribution: {}", e))
}

#[tauri::command]
pub fn delete_goal(db: State<Database>, id: i64) -> Result<(), String> {
    db.delete_goal(id).map_err(|e| format!("Failed to delete goal: {}", e))
}

// ========== PROJECT CACHE COMMANDS ==========

#[tauri::command]
pub fn get_projects_by_type_id(db: State<Database>, session_type_id: i64) -> Result<Vec<String>, String> {
    db.get_projects_by_type_id(session_type_id).map_err(|e| format!("Failed to get projects: {}", e))
}

// ========== DASHBOARD / ANALYTICS COMMANDS ==========

#[tauri::command]
pub fn get_today_summary(db: State<Database>) -> Result<TodaySummary, String> {
    let today = logic::get_today();
    let sessions = db.get_sessions_by_date(&today).map_err(|e| format!("Failed to get today's sessions: {}", e))?;
    let session_types = db.get_all_session_types().map_err(|e| format!("Failed to get session types: {}", e))?;

    let total_hours: f64 = sessions.iter().map(|s| s.hours).sum();
    let total_pay: f64 = sessions.iter().map(|s| s.calculate_pay()).sum();

    let mut session_hours: HashMap<String, f64> = HashMap::new();
    for st in &session_types {
        let hours: f64 = sessions.iter()
            .filter(|s| s.session_type_id == st.id)
            .map(|s| s.hours)
            .sum();
        session_hours.insert(st.name.clone(), hours);
    }

    Ok(TodaySummary {
        date: today,
        total_hours,
        total_pay,
        session_hours,
    })
}

#[tauri::command]
pub fn get_pay_summary(db: State<Database>) -> Result<PaySummary, String> {
    db.get_pay_summary().map_err(|e| format!("Failed to get pay summary: {}", e))
}

#[tauri::command]
pub fn get_recent_sessions(db: State<Database>, limit: usize) -> Result<Vec<WorkSession>, String> {
    let all_sessions = db.get_all_sessions().map_err(|e| format!("Failed to get sessions: {}", e))?;
    Ok(all_sessions.into_iter().take(limit).collect())
}

#[tauri::command]
pub fn calculate_avg_weekly_income(db: State<Database>) -> Result<f64, String> {
    let sessions = db.get_all_sessions().map_err(|e| format!("Failed to get sessions: {}", e))?;
    Ok(logic::calculate_avg_weekly_income(&sessions))
}

#[tauri::command]
pub fn get_goal_eta(db: State<Database>, goal_id: i64) -> Result<Option<String>, String> {
    let goals = db.get_all_goals().map_err(|e| format!("Failed to get goals: {}", e))?;
    let goal = goals.iter().find(|g| g.id == goal_id).ok_or_else(|| "Goal not found".to_string())?;
    let sessions = db.get_all_sessions().map_err(|e| format!("Failed to get sessions: {}", e))?;
    let avg_income = logic::calculate_avg_weekly_income(&sessions);
    Ok(logic::calculate_goal_eta(goal, avg_income))
}

#[tauri::command]
pub fn get_current_date() -> String {
    logic::get_today()
}

// ========== ANALYTICS COMMANDS ==========

#[tauri::command]
pub fn get_analytics(db: State<Database>, range_start: String, range_end: String) -> Result<AnalyticsData, String> {
    db.get_analytics(&range_start, &range_end)
        .map_err(|e| format!("Failed to get analytics: {}", e))
}

// ========== AI ADVISOR COMMANDS ==========

#[tauri::command]
pub async fn ask_ai_advisor(
    db: State<'_, Database>,
    question: String,
    context: AIContext,
) -> Result<AIResponse, String> {
    // Get license info from settings
    let license_key = db.get_setting("license_key")
        .map_err(|e| format!("Failed to get license: {}", e))?
        .unwrap_or_default();
    
    let device_id = db.get_setting("device_id")
        .map_err(|e| format!("Failed to get device ID: {}", e))?
        .unwrap_or_default();
    
    if license_key.is_empty() || device_id.is_empty() {
        return Err("AI Advisor requires a valid license. Please activate your Pro license in Settings.".to_string());
    }
    
    let client = reqwest::Client::new();
    
    let body = serde_json::json!({
        "license_key": license_key,
        "device_id": device_id,
        "question": question,
        "context": context
    });
    
    let response = client
        .post("https://backend-ten-silk-91.vercel.app/api/ai/chat")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to AI service: {}", e))?;
    
    let status = response.status();
    let result: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    if !status.is_success() {
        let error = result["error"].as_str().unwrap_or("Unknown error");
        return Err(error.to_string());
    }
    
    let content = result["data"]["response"].as_str()
        .unwrap_or("Sorry, I couldn't generate a response.")
        .to_string();
    
    let remaining = result["data"]["usage"]["remaining"].as_i64().unwrap_or(0) as i32;
    let limit = result["data"]["usage"]["limit"].as_i64().unwrap_or(10) as i32;
    
    Ok(AIResponse {
        content,
        remaining_queries: remaining,
        daily_limit: limit,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AIResponse {
    pub content: String,
    pub remaining_queries: i32,
    pub daily_limit: i32,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AIContext {
    pub today_hours: f64,
    pub today_pay: f64,
    pub today_sessions: i32,
    
    pub period_total_hours: f64,
    pub period_total_sessions: i32,
    pub period_total_pay: f64,
    pub period_avg_session: f64,
    pub period_longest_session: f64,
    
    pub avg_weekly_income: f64,
    pub avg_daily_hours: f64,
    pub consistency_score: i32,
    
    pub categories_summary: String,
    
    pub best_weekday: String,
    pub worst_weekday: String,
    pub weekday_summary: String,
    
    pub goals_count: i32,
    pub goals_summary: String,
    pub total_debt: f64,
    pub total_savings_target: f64,
    
    pub recent_sessions_summary: String,
}

// ========== HELPER STRUCTS ==========

#[derive(Debug, Clone, serde::Serialize)]
pub struct TodaySummary {
    pub date: String,
    pub total_hours: f64,
    pub total_pay: f64,
    pub session_hours: HashMap<String, f64>,
}

// ========== ACHIEVEMENT COMMANDS ==========

#[tauri::command]
pub fn get_achievements(db: State<Database>) -> Result<Vec<Achievement>, String> {
    let unlocked = db.get_unlocked_achievements()
        .map_err(|e| format!("Failed to get achievements: {}", e))?;
    
    let unlocked_map: std::collections::HashMap<String, String> = unlocked.into_iter().collect();
    
    let achievements: Vec<Achievement> = ACHIEVEMENTS.iter().map(|def| {
        let unlocked_at = unlocked_map.get(def.id).cloned();
        Achievement {
            id: def.id.to_string(),
            name: def.name.to_string(),
            description: def.description.to_string(),
            category: def.category,
            icon: def.icon.to_string(),
            unlocked: unlocked_at.is_some(),
            unlocked_at,
        }
    }).collect();
    
    Ok(achievements)
}

#[tauri::command]
pub fn check_and_unlock_achievements(db: State<Database>) -> Result<Vec<String>, String> {
    let mut newly_unlocked: Vec<String> = Vec::new();
    
    // Get current unlocked set
    let unlocked = db.get_unlocked_achievements()
        .map_err(|e| format!("Failed to get achievements: {}", e))?;
    let unlocked_ids: std::collections::HashSet<String> = unlocked.into_iter().map(|(id, _)| id).collect();
    
    // Check each achievement
    for def in ACHIEVEMENTS {
        if unlocked_ids.contains(def.id) {
            continue; // Already unlocked
        }
        
        let should_unlock = check_achievement_condition(&db, def.id)
            .map_err(|e| format!("Failed to check achievement {}: {}", def.id, e))?;
        
        if should_unlock {
            let is_new = db.unlock_achievement(def.id)
                .map_err(|e| format!("Failed to unlock achievement: {}", e))?;
            if is_new {
                newly_unlocked.push(def.id.to_string());
            }
        }
    }
    
    Ok(newly_unlocked)
}

#[tauri::command]
pub fn log_app_event(db: State<Database>, event_type: String, event_data: Option<String>) -> Result<(), String> {
    db.log_app_event(&event_type, event_data.as_deref())
        .map_err(|e| format!("Failed to log event: {}", e))
}

fn check_achievement_condition(db: &Database, achievement_id: &str) -> Result<bool, String> {
    let result = match achievement_id {
        // Presence
        "first_step" => db.count_total_sessions().map(|c| c >= 1),
        "back_again" => db.count_distinct_session_days().map(|c| c >= 3),
        "getting_comfortable" => db.count_total_sessions().map(|c| c >= 10),
        "part_of_routine" => db.count_distinct_session_days().map(|c| c >= 7),
        
        // Awareness
        "curious_mind" => db.count_event_days("view_analytics").map(|c| c >= 1),
        "pattern_noticed" => db.count_event_days("view_analytics").map(|c| c >= 5),
        "zoomed_out" => db.count_distinct_event_data("analytics_range").map(|c| c >= 3),
        "connecting_dots" => db.events_same_day("view_analytics", "view_advisor"),
        
        // Balance
        "paced_yourself" => db.has_paced_week(),
        "sustainable_week" => db.has_sustainable_week(),
        "human_weekend" => db.has_human_weekend(),
        
        // Commitment
        "long_run" => db.count_distinct_session_weeks().map(|c| c >= 3),
        "one_full_month" => db.count_distinct_session_weeks().map(|c| c >= 4),
        "hundred_hours" => db.get_total_hours().map(|h| h >= 100.0),
        
        // Financial
        "first_dollar" => db.has_paid_session(),
        
        _ => Ok(false),
    };
    
    result.map_err(|e| e.to_string())
}

// ========== FINANCIAL SIMULATOR COMMANDS ==========

#[tauri::command]
pub fn simulate_financial_scenario(
    db: State<Database>,
    hours_per_week: f64,
    hourly_rate: f64,
    weekly_expenses: f64,
) -> Result<SimulationResult, String> {
    let weekly_income = hours_per_week * hourly_rate;
    let weekly_savings = weekly_income - weekly_expenses;
    
    // Get all goals for projections
    let goals = db.get_all_goals().map_err(|e| format!("Failed to get goals: {}", e))?;
    
    let mut goal_projections: Vec<GoalProjection> = Vec::new();
    for goal in goals {
        let remaining = goal.remaining_amount();
        let weeks_to_complete = if weekly_savings > 0.0 {
            Some(remaining / weekly_savings)
        } else {
            None
        };
        
        let completion_date = weeks_to_complete.map(|weeks| {
            let days = (weeks * 7.0) as i64;
            let now = chrono::Local::now();
            let completion = now + chrono::Duration::days(days);
            completion.format("%Y-%m-%d").to_string()
        });
        
        goal_projections.push(GoalProjection {
            goal_id: goal.id,
            goal_name: goal.name.clone(),
            remaining,
            weeks_to_complete,
            completion_date,
        });
    }
    
    // Calculate sustainability score
    let sustainability_score = if weekly_income == 0.0 {
        0.0
    } else if weekly_savings <= 0.0 {
        (weekly_income / weekly_expenses * 50.0).min(50.0)
    } else {
        let savings_rate = weekly_savings / weekly_income;
        (50.0 + savings_rate * 50.0).min(100.0)
    };
    
    // Generate insights
    let mut insights: Vec<String> = Vec::new();
    
    if weekly_savings < 0.0 {
        insights.push(format!(
            "⚠️ Deficit of ${:.2}/week. You need ${:.2} more income or reduce expenses.",
            -weekly_savings, -weekly_savings
        ));
    } else if weekly_savings < weekly_income * 0.1 {
        insights.push("💡 Savings rate under 10%. Consider reducing expenses.".to_string());
    } else if weekly_savings > weekly_income * 0.3 {
        insights.push("✅ Great savings rate! Over 30% of income saved.".to_string());
    }
    
    if hours_per_week > 50.0 {
        insights.push("⚠️ Working over 50 hrs/week risks burnout.".to_string());
    } else if hours_per_week < 20.0 && weekly_savings < 0.0 {
        insights.push("💡 Consider increasing hours to close the deficit.".to_string());
    }
    
    if hourly_rate < 25.0 && hours_per_week > 40.0 {
        insights.push("💡 Low rate + long hours. Consider raising your rate.".to_string());
    }
    
    Ok(SimulationResult {
        weekly_income,
        weekly_savings,
        monthly_income: weekly_income * 4.33,
        monthly_savings: weekly_savings * 4.33,
        yearly_income: weekly_income * 52.0,
        yearly_savings: weekly_savings * 52.0,
        goal_projections,
        sustainability_score,
        insights,
    })
}

#[tauri::command]
pub fn get_current_financial_baseline(db: State<Database>) -> Result<(f64, f64), String> {
    let sessions = db.get_all_sessions().map_err(|e| format!("Failed: {}", e))?;
    let avg_weekly_hours = logic::calculate_avg_weekly_hours(&sessions);
    let avg_weekly_income = logic::calculate_avg_weekly_income(&sessions);
    let avg_hourly_rate = if avg_weekly_hours > 0.0 {
        avg_weekly_income / avg_weekly_hours
    } else {
        30.0 // Default
    };
    Ok((avg_weekly_hours, avg_hourly_rate))
}

// ========== FOCUS & BURNOUT COMMANDS ==========

#[tauri::command]
pub fn get_focus_metrics(db: State<Database>, days: i32) -> Result<FocusMetrics, String> {
    let end_date = chrono::Local::now();
    let start_date = end_date - chrono::Duration::days(days as i64);
    let start_str = start_date.format("%Y-%m-%d").to_string();
    let end_str = end_date.format("%Y-%m-%d").to_string();
    
    let analytics = db.get_analytics(&start_str, &end_str)
        .map_err(|e| format!("Failed: {}", e))?;
    
    let session_count = analytics.summary.total_sessions;
    let total_hours = analytics.summary.total_hours;
    let avg_session_length = analytics.summary.avg_session_length;
    
    // Calculate fragmentation (many short sessions = fragmented)
    let short_session_threshold = 1.0; // hours
    let sessions = db.get_all_sessions().map_err(|e| format!("Failed: {}", e))?;
    let short_sessions = sessions.iter()
        .filter(|s| s.date >= start_str && s.date <= end_str)
        .filter(|s| s.hours < short_session_threshold)
        .count();
    
    let fragmentation_score = if session_count > 0 {
        100.0 - (short_sessions as f64 / session_count as f64 * 100.0)
    } else {
        100.0
    };
    
    // Calculate streaks
    let (current_streak, longest_streak) = calculate_streaks(&sessions);
    
    // Focus score combines avg session length, fragmentation, consistency
    let focus_score = (
        (avg_session_length / 4.0 * 40.0).min(40.0) + // Up to 40 points for good session length
        fragmentation_score * 0.3 +                   // Up to 30 points for low fragmentation  
        (current_streak as f64 / 7.0 * 30.0).min(30.0) // Up to 30 points for streak
    ).min(100.0);
    
    Ok(FocusMetrics {
        focus_score,
        fragmentation_score,
        avg_session_length,
        session_count,
        total_hours,
        longest_streak_days: longest_streak,
        current_streak_days: current_streak,
    })
}

fn calculate_streaks(sessions: &[WorkSession]) -> (i32, i32) {
    if sessions.is_empty() {
        return (0, 0);
    }
    
    let mut dates: std::collections::HashSet<String> = std::collections::HashSet::new();
    for s in sessions {
        dates.insert(s.date.clone());
    }
    
    let mut dates_vec: Vec<_> = dates.into_iter().collect();
    dates_vec.sort();
    
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let yesterday = (chrono::Local::now() - chrono::Duration::days(1))
        .format("%Y-%m-%d").to_string();
    
    // Current streak
    let mut current_streak = 0;
    let mut check_date = chrono::Local::now().date_naive();
    
    // Allow starting from today or yesterday
    if dates_vec.contains(&today) || dates_vec.contains(&yesterday) {
        while dates_vec.contains(&check_date.format("%Y-%m-%d").to_string()) {
            current_streak += 1;
            check_date = check_date.pred_opt().unwrap_or(check_date);
        }
    }
    
    // Longest streak
    let mut longest_streak = 0;
    let mut streak = 1;
    for i in 1..dates_vec.len() {
        if let (Ok(prev), Ok(curr)) = (
            chrono::NaiveDate::parse_from_str(&dates_vec[i-1], "%Y-%m-%d"),
            chrono::NaiveDate::parse_from_str(&dates_vec[i], "%Y-%m-%d"),
        ) {
            if (curr - prev).num_days() == 1 {
                streak += 1;
            } else {
                longest_streak = longest_streak.max(streak);
                streak = 1;
            }
        }
    }
    longest_streak = longest_streak.max(streak);
    
    (current_streak, longest_streak)
}

#[tauri::command]
pub fn get_burnout_risk(db: State<Database>, days: i64) -> Result<BurnoutRisk, String> {
    let end_date = chrono::Local::now();
    let start_date = end_date - chrono::Duration::days(days);
    let start_str = start_date.format("%Y-%m-%d").to_string();
    let end_str = end_date.format("%Y-%m-%d").to_string();
    
    let sessions = db.get_all_sessions().map_err(|e| format!("Failed: {}", e))?;
    let recent_sessions: Vec<_> = sessions.iter()
        .filter(|s| s.date >= start_str && s.date <= end_str)
        .collect();
    
    let mut factors: Vec<BurnoutFactor> = Vec::new();
    let mut risk_score: f64 = 0.0;
    
    // Factor 1: Weekly hours
    let total_hours: f64 = recent_sessions.iter().map(|s| s.hours).sum();
    let weekly_hours = total_hours / 2.0; // 2 weeks
    
    let (hours_severity, hours_points) = if weekly_hours > 60.0 {
        ("danger", 35.0)
    } else if weekly_hours > 50.0 {
        ("warning", 20.0)
    } else {
        ("ok", 0.0)
    };
    
    factors.push(BurnoutFactor {
        name: "Weekly Hours".to_string(),
        severity: hours_severity.to_string(),
        value: format!("{:.1}h/week", weekly_hours),
        threshold: "Under 50h".to_string(),
    });
    risk_score += hours_points;
    
    // Factor 2: Longest session
    let max_session = recent_sessions.iter().map(|s| s.hours).fold(0.0_f64, f64::max);
    let (max_severity, max_points) = if max_session > 10.0 {
        ("danger", 25.0)
    } else if max_session > 8.0 {
        ("warning", 15.0)
    } else {
        ("ok", 0.0)
    };
    
    factors.push(BurnoutFactor {
        name: "Longest Session".to_string(),
        severity: max_severity.to_string(),
        value: format!("{:.1}h", max_session),
        threshold: "Under 8h".to_string(),
    });
    risk_score += max_points;
    
    // Factor 3: Rest days
    let unique_days: std::collections::HashSet<&String> = recent_sessions.iter()
        .map(|s| &s.date)
        .collect();
    let rest_days = 14 - unique_days.len() as i32;
    
    let (rest_severity, rest_points) = if rest_days < 2 {
        ("danger", 30.0)
    } else if rest_days < 4 {
        ("warning", 15.0)
    } else {
        ("ok", 0.0)
    };
    
    factors.push(BurnoutFactor {
        name: "Rest Days (2 weeks)".to_string(),
        severity: rest_severity.to_string(),
        value: format!("{} days", rest_days),
        threshold: "At least 4 days".to_string(),
    });
    risk_score += rest_points;
    
    // Factor 4: Work pattern consistency
    let hours_per_day: std::collections::HashMap<&String, f64> = recent_sessions.iter()
        .fold(std::collections::HashMap::new(), |mut acc, s| {
            *acc.entry(&s.date).or_insert(0.0) += s.hours;
            acc
        });
    
    let variance = if !hours_per_day.is_empty() {
        let mean: f64 = hours_per_day.values().sum::<f64>() / hours_per_day.len() as f64;
        let var: f64 = hours_per_day.values()
            .map(|h| (h - mean).powi(2))
            .sum::<f64>() / hours_per_day.len() as f64;
        var.sqrt()
    } else {
        0.0
    };
    
    let (var_severity, var_points) = if variance > 4.0 {
        ("warning", 10.0)
    } else {
        ("ok", 0.0)
    };
    
    factors.push(BurnoutFactor {
        name: "Schedule Consistency".to_string(),
        severity: var_severity.to_string(),
        value: format!("±{:.1}h variation", variance),
        threshold: "Under ±4h".to_string(),
    });
    risk_score += var_points;
    
    // Determine risk level
    let risk_level = if risk_score >= 70.0 {
        "Critical"
    } else if risk_score >= 45.0 {
        "High"
    } else if risk_score >= 20.0 {
        "Moderate"
    } else {
        "Low"
    };
    
    // Generate recommendations
    let mut recommendations: Vec<String> = Vec::new();
    
    if hours_severity != "ok" {
        recommendations.push("Reduce weekly hours to under 50 for sustainability.".to_string());
    }
    if max_severity != "ok" {
        recommendations.push("Break long sessions into smaller blocks with breaks.".to_string());
    }
    if rest_severity != "ok" {
        recommendations.push("Schedule at least 2 full rest days per week.".to_string());
    }
    if var_severity != "ok" {
        recommendations.push("Establish a more consistent daily schedule.".to_string());
    }
    
    if recommendations.is_empty() {
        recommendations.push("Great work-life balance! Keep it up.".to_string());
    }
    
    Ok(BurnoutRisk {
        risk_level: risk_level.to_string(),
        risk_score,
        factors,
        recommendations,
    })
}

// ========== INVOICE COMMANDS ==========

#[tauri::command]
pub fn get_all_invoices(db: State<Database>) -> Result<Vec<Invoice>, String> {
    db.get_all_invoices().map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn get_invoice(db: State<Database>, id: i64) -> Result<Invoice, String> {
    db.get_invoice(id).map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn create_invoice(db: State<Database>, invoice: NewInvoice) -> Result<i64, String> {
    db.create_invoice(invoice).map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn update_invoice_status(db: State<Database>, id: i64, status: String) -> Result<(), String> {
    let status = InvoiceStatus::from_string(&status)?;
    db.update_invoice_status(id, status).map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn delete_invoice(db: State<Database>, id: i64) -> Result<(), String> {
    db.delete_invoice(id).map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn get_uninvoiced_sessions(db: State<Database>) -> Result<Vec<WorkSession>, String> {
    db.get_uninvoiced_sessions().map_err(|e| format!("Failed: {}", e))
}

// ========== HABIT COMMANDS ==========

#[tauri::command]
pub fn get_all_habits(db: State<Database>) -> Result<Vec<Habit>, String> {
    db.get_all_habits().map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn add_habit(db: State<Database>, habit: Habit) -> Result<i64, String> {
    db.add_habit(habit).map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn update_habit(db: State<Database>, habit: Habit) -> Result<(), String> {
    db.update_habit(&habit).map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn delete_habit(db: State<Database>, id: i64) -> Result<(), String> {
    db.delete_habit(id).map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn log_habit_completion(db: State<Database>, habit_id: i64, notes: Option<String>) -> Result<i64, String> {
    db.log_habit_completion(habit_id, notes).map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn get_pending_habits(db: State<Database>) -> Result<Vec<Habit>, String> {
    // Get habits that should be triggered based on today's work
    let today = logic::get_today();
    let sessions = db.get_sessions_by_date(&today).map_err(|e| format!("Failed: {}", e))?;
    let total_hours: f64 = sessions.iter().map(|s| s.hours).sum();
    let session_count = sessions.len();
    
    let habits = db.get_all_habits().map_err(|e| format!("Failed: {}", e))?;
    let completed_today = db.get_habit_logs_for_date(&today).map_err(|e| format!("Failed: {}", e))?;
    let completed_ids: std::collections::HashSet<i64> = completed_today.iter().map(|l| l.habit_id).collect();
    
    let pending: Vec<Habit> = habits.into_iter()
        .filter(|h| h.is_active && !completed_ids.contains(&h.id))
        .filter(|h| {
            match h.trigger_type.as_str() {
                "after_session" => session_count as f64 >= h.trigger_value,
                "after_hours" => total_hours >= h.trigger_value,
                "daily" => true,
                _ => false,
            }
        })
        .collect();
    
    Ok(pending)
}

// ========== AUTO-TRACKING COMMANDS ==========

#[tauri::command]
pub fn get_activity_suggestions(db: State<Database>) -> Result<Vec<ActivitySuggestion>, String> {
    db.get_pending_suggestions().map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn accept_activity_suggestion(
    db: State<Database>,
    suggestion_id: i64,
    session_type_id: i64,
    project_name: String,
) -> Result<i64, String> {
    db.accept_suggestion(suggestion_id, session_type_id, &project_name)
        .map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn dismiss_activity_suggestion(db: State<Database>, suggestion_id: i64) -> Result<(), String> {
    db.dismiss_suggestion(suggestion_id).map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn get_tracking_rules(db: State<Database>) -> Result<Vec<TrackingRule>, String> {
    db.get_tracking_rules().map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn add_tracking_rule(db: State<Database>, rule: TrackingRule) -> Result<i64, String> {
    db.add_tracking_rule(rule).map_err(|e| format!("Failed: {}", e))
}

#[tauri::command]
pub fn delete_tracking_rule(db: State<Database>, id: i64) -> Result<(), String> {
    db.delete_tracking_rule(id).map_err(|e| format!("Failed: {}", e))
}

// ========== ENHANCED AI ADVISOR COMMANDS ==========

#[tauri::command]
pub fn get_financial_analysis(db: State<Database>) -> Result<FinancialAnalysis, String> {
    let sessions = db.get_all_sessions().map_err(|e| format!("Failed: {}", e))?;
    let goals = db.get_all_goals().map_err(|e| format!("Failed: {}", e))?;
    
    let avg_weekly_hours = logic::calculate_avg_weekly_hours(&sessions);
    let avg_weekly_income = logic::calculate_avg_weekly_income(&sessions);
    
    // Calculate trend (last 4 weeks vs previous 4 weeks)
    let now = chrono::Local::now();
    let four_weeks_ago = now - chrono::Duration::weeks(4);
    let eight_weeks_ago = now - chrono::Duration::weeks(8);
    
    let recent_income: f64 = sessions.iter()
        .filter(|s| {
            chrono::NaiveDate::parse_from_str(&s.date, "%Y-%m-%d")
                .map(|d| d > four_weeks_ago.date_naive())
                .unwrap_or(false)
        })
        .map(|s| s.calculate_pay())
        .sum();
        
    let older_income: f64 = sessions.iter()
        .filter(|s| {
            chrono::NaiveDate::parse_from_str(&s.date, "%Y-%m-%d")
                .map(|d| d > eight_weeks_ago.date_naive() && d <= four_weeks_ago.date_naive())
                .unwrap_or(false)
        })
        .map(|s| s.calculate_pay())
        .sum();
    
    let income_trend = if recent_income > older_income * 1.1 {
        "increasing"
    } else if recent_income < older_income * 0.9 {
        "decreasing"
    } else {
        "stable"
    };
    
    // Generate insights
    let mut insights: Vec<FinancialInsight> = Vec::new();
    
    // Income insight
    if avg_weekly_income > 0.0 {
        let severity = if income_trend == "increasing" { "success" } else if income_trend == "decreasing" { "warning" } else { "info" };
        insights.push(FinancialInsight {
            category: "income".to_string(),
            severity: severity.to_string(),
            title: format!("Income {} over past 8 weeks", income_trend),
            message: format!("Recent 4 weeks: ${:.0} | Previous 4 weeks: ${:.0}", recent_income, older_income),
            action: if income_trend == "decreasing" { Some("Consider taking on more hours or increasing rates.".to_string()) } else { None },
        });
    }
    
    // Goal progress insights
    for goal in &goals {
        let progress = goal.progress_percent();
        if progress >= 90.0 && progress < 100.0 {
            insights.push(FinancialInsight {
                category: "goals".to_string(),
                severity: "success".to_string(),
                title: format!("{} almost complete!", goal.name),
                message: format!("{:.0}% complete - only ${:.2} to go!", progress, goal.remaining_amount()),
                action: None,
            });
        }
    }
    
    // Sustainability insight
    if avg_weekly_hours > 50.0 {
        insights.push(FinancialInsight {
            category: "sustainability".to_string(),
            severity: "warning".to_string(),
            title: "High weekly hours".to_string(),
            message: format!("Averaging {:.1} hours/week. Consider sustainable pacing.", avg_weekly_hours),
            action: Some("Review if all tasks are necessary or if rates could increase.".to_string()),
        });
    }
    
    // Recommendations
    let mut recommendations: Vec<String> = Vec::new();
    
    if avg_weekly_income > 0.0 && !goals.is_empty() {
        let total_remaining: f64 = goals.iter().map(|g| g.remaining_amount()).sum();
        let weeks_to_clear = total_remaining / avg_weekly_income;
        recommendations.push(format!(
            "At current pace, all goals complete in ~{:.0} weeks (${:.2} remaining)",
            weeks_to_clear, total_remaining
        ));
    }
    
    if income_trend == "decreasing" {
        recommendations.push("Income trending down. Review project pipeline.".to_string());
    }
    
    Ok(FinancialAnalysis {
        avg_weekly_hours,
        avg_weekly_income,
        projected_monthly_income: avg_weekly_income * 4.33,
        projected_yearly_income: avg_weekly_income * 52.0,
        income_trend: income_trend.to_string(),
        insights,
        recommendations,
    })
}

// ========== LICENSE COMMANDS ==========

#[tauri::command]
pub fn get_license(db: State<Database>) -> Result<License, String> {
    db.get_license().map_err(|e| format!("Failed to get license: {}", e))
}

#[tauri::command]
pub fn get_feature_limits(db: State<Database>) -> Result<FeatureLimits, String> {
    let license = db.get_license().map_err(|e| format!("Failed to get license: {}", e))?;
    Ok(FeatureLimits::for_tier(license.tier))
}

#[tauri::command]
pub fn get_current_usage(db: State<Database>) -> Result<CurrentUsage, String> {
    let session_type_count = db.get_session_type_count()
        .map_err(|e| format!("Failed to get session type count: {}", e))?;
    let goal_count = db.get_goal_count()
        .map_err(|e| format!("Failed to get goal count: {}", e))?;
    
    Ok(CurrentUsage {
        session_type_count,
        goal_count,
    })
}

#[tauri::command]
pub async fn activate_license(db: State<'_, Database>, license_key: String) -> Result<License, String> {
    // Get or create device ID
    let device_id = db.get_setting("device_id")
        .map_err(|e| format!("Failed to get device ID: {}", e))?
        .unwrap_or_else(|| {
            let new_id = uuid::Uuid::new_v4().to_string();
            let _ = db.set_setting("device_id", &new_id);
            new_id
        });
    
    // Validate with backend
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "license_key": license_key.trim(),
        "device_id": device_id,
        "device_name": hostname::get().ok().and_then(|h| h.into_string().ok()).unwrap_or_else(|| "Unknown".to_string())
    });
    
    let response = client
        .post("https://backend-ten-silk-91.vercel.app/api/license/activate")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to license server: {}", e))?;
    
    let status = response.status();
    let result: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    if !status.is_success() {
        let error = result["error"].as_str().unwrap_or("License activation failed");
        return Err(error.to_string());
    }
    
    // Parse tier from response (tier is at top level, not nested in "data")
    let tier_str = result["tier"].as_str().unwrap_or("free");
    let tier = match tier_str {
        "lifetime" => Tier::Lifetime,
        "pro" => Tier::Pro,
        _ => Tier::Free,
    };
    
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let license = License {
        tier,
        license_key: Some(license_key),
        activated_at: Some(now),
        expires_at: None,
    };
    
    // Save to local DB
    db.save_license(&license).map_err(|e| format!("Failed to save license: {}", e))?;
    
    Ok(license)
}

#[tauri::command]
pub fn deactivate_license(db: State<Database>) -> Result<License, String> {
    let license = License::default();
    db.save_license(&license).map_err(|e| format!("Failed to save license: {}", e))?;
    Ok(license)
}

/// Check if user can create more session types
#[tauri::command]
pub fn can_create_session_type(db: State<Database>) -> Result<LimitCheck, String> {
    let license = db.get_license().map_err(|e| format!("Failed to get license: {}", e))?;
    let limits = FeatureLimits::for_tier(license.tier);
    let count = db.get_session_type_count().map_err(|e| format!("Failed to get count: {}", e))?;
    
    let allowed = match limits.max_session_types {
        Some(max) => count < max,
        None => true,
    };
    
    Ok(LimitCheck {
        allowed,
        current: count,
        limit: limits.max_session_types,
        feature: "session_types".to_string(),
    })
}

/// Check if user can create more goals
#[tauri::command]
pub fn can_create_goal(db: State<Database>) -> Result<LimitCheck, String> {
    let license = db.get_license().map_err(|e| format!("Failed to get license: {}", e))?;
    let limits = FeatureLimits::for_tier(license.tier);
    let count = db.get_goal_count().map_err(|e| format!("Failed to get count: {}", e))?;
    
    let allowed = match limits.max_goals {
        Some(max) => count < max,
        None => true,
    };
    
    Ok(LimitCheck {
        allowed,
        current: count,
        limit: limits.max_goals,
        feature: "goals".to_string(),
    })
}

// Helper structs for commands
#[derive(serde::Serialize)]
pub struct CurrentUsage {
    pub session_type_count: u32,
    pub goal_count: u32,
}

#[derive(serde::Serialize)]
pub struct LimitCheck {
    pub allowed: bool,
    pub current: u32,
    pub limit: Option<u32>,
    pub feature: String,
}
