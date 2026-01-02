use crate::core::models::*;
use chrono::Local;

pub fn get_today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn calculate_goal_eta(goal: &FinancialGoal, avg_weekly_income: f64) -> Option<String> {
    if avg_weekly_income <= 0.0 {
        return None;
    }

    let remaining = goal.remaining_amount();
    if remaining <= 0.0 {
        return Some("Goal Complete!".to_string());
    }

    let weeks_needed = (remaining / avg_weekly_income).ceil() as i64;

    if weeks_needed < 4 {
        Some(format!("{} weeks", weeks_needed))
    } else if weeks_needed < 52 {
        let months = (weeks_needed as f64 / 4.33).ceil() as i64;
        Some(format!("{} months", months))
    } else {
        let years = (weeks_needed as f64 / 52.0).ceil() as i64;
        Some(format!("{} years", years))
    }
}

pub fn total_hours_for_date(sessions: &[WorkSession], date: &str) -> f64 {
    sessions.iter().filter(|s| s.date == date).map(|s| s.hours).sum()
}

pub fn total_pay_for_date(sessions: &[WorkSession], date: &str) -> f64 {
    sessions.iter().filter(|s| s.date == date).map(|s| s.calculate_pay()).sum()
}

pub fn calculate_avg_weekly_income(sessions: &[WorkSession]) -> f64 {
    let today = Local::now();
    let four_weeks_ago = today - chrono::Duration::weeks(4);

    let total_pay: f64 = sessions
        .iter()
        .filter(|s| {
            if let Ok(session_date) = chrono::NaiveDate::parse_from_str(&s.date, "%Y-%m-%d") {
                let session_datetime = session_date.and_hms_opt(0, 0, 0).unwrap();
                session_datetime >= four_weeks_ago.naive_local()
            } else {
                false
            }
        })
        .map(|s| s.calculate_pay())
        .sum();

    total_pay / 4.0
}

pub fn calculate_avg_weekly_hours(sessions: &[WorkSession]) -> f64 {
    let today = Local::now();
    let four_weeks_ago = today - chrono::Duration::weeks(4);

    let total_hours: f64 = sessions
        .iter()
        .filter(|s| {
            if let Ok(session_date) = chrono::NaiveDate::parse_from_str(&s.date, "%Y-%m-%d") {
                let session_datetime = session_date.and_hms_opt(0, 0, 0).unwrap();
                session_datetime >= four_weeks_ago.naive_local()
            } else {
                false
            }
        })
        .map(|s| s.hours)
        .sum();

    total_hours / 4.0
}
