#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod storage;
mod commands;

use storage::db::Database;
use std::path::PathBuf;

fn main() {
    let app_data_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
        .unwrap_or_else(|| PathBuf::from("."));

    let db_path = app_data_dir.join("chrono.db");

    let database = Database::new(db_path)
        .expect("Failed to initialize database");

    tauri::Builder::default()
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            // Session type commands
            commands::get_all_session_types,
            commands::add_session_type,
            commands::update_session_type,
            commands::delete_session_type,
            // Session commands
            commands::get_all_sessions,
            commands::get_sessions_by_date,
            commands::get_sessions_by_type_id,
            commands::add_session,
            commands::update_session,
            commands::delete_session,
            // Goal commands
            commands::get_all_goals,
            commands::add_goal,
            commands::update_goal,
            commands::add_contribution,
            commands::delete_goal,
            // Project commands
            commands::get_projects_by_type_id,
            // Dashboard commands
            commands::get_today_summary,
            commands::get_pay_summary,
            commands::get_recent_sessions,
            commands::calculate_avg_weekly_income,
            commands::get_goal_eta,
            commands::get_current_date,
            // Analytics commands
            commands::get_analytics,
            // AI Advisor
            commands::ask_ai_advisor,
            // Achievement commands
            commands::get_achievements,
            commands::check_and_unlock_achievements,
            commands::log_app_event,
            // Financial Simulator
            commands::simulate_financial_scenario,
            commands::get_current_financial_baseline,
            // Focus & Burnout
            commands::get_focus_metrics,
            commands::get_burnout_risk,
            // Invoice commands
            commands::get_all_invoices,
            commands::get_invoice,
            commands::create_invoice,
            commands::update_invoice_status,
            commands::delete_invoice,
            commands::get_uninvoiced_sessions,
            // Habit commands
            commands::get_all_habits,
            commands::add_habit,
            commands::update_habit,
            commands::delete_habit,
            commands::log_habit_completion,
            commands::get_pending_habits,
            // Auto-tracking commands
            commands::get_activity_suggestions,
            commands::accept_activity_suggestion,
            commands::dismiss_activity_suggestion,
            commands::get_tracking_rules,
            commands::add_tracking_rule,
            commands::delete_tracking_rule,
            // Enhanced AI Advisor
            commands::get_financial_analysis,
            // License commands
            commands::get_license,
            commands::get_feature_limits,
            commands::get_current_usage,
            commands::activate_license,
            commands::deactivate_license,
            commands::can_create_session_type,
            commands::can_create_goal,
            // Updater commands
            commands::check_for_update,
            commands::install_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
