use rusqlite::{Connection, Result as SqlResult, params};
use std::path::PathBuf;
use std::sync::Mutex;
use crate::core::models::*;
use chrono::Datelike;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: PathBuf) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch(include_str!("schema.sql"))?;
        Self::run_migrations(&conn)?;
        Self::seed_default_session_types(&conn)?;
        Ok(Database { conn: Mutex::new(conn) })
    }

    fn run_migrations(conn: &Connection) -> SqlResult<()> {
        // Migration from old schema to new schema
        let has_old_schema: bool = conn
            .prepare("SELECT session_type FROM sessions LIMIT 1")
            .is_ok();

        if has_old_schema {
            // Check if session_types table exists
            let has_session_types: i64 = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='session_types'",
                [],
                |row| row.get(0)
            )?;

            if has_session_types == 0 {
                return Ok(()); // Old schema exists but no session_types table - skip migration
            }
        }
        Ok(())
    }

    fn seed_default_session_types(conn: &Connection) -> SqlResult<()> {
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM session_types", [], |row| row.get(0))?;
        if count == 0 {
            conn.execute(
                "INSERT INTO session_types (name, color, hourly_rate) VALUES ('Work', '#22C55E', 30.0)",
                [],
            )?;
            conn.execute(
                "INSERT INTO session_types (name, color, hourly_rate) VALUES ('Study', '#3B82F6', NULL)",
                [],
            )?;
        }
        Ok(())
    }

    // ========== SESSION TYPE OPERATIONS ==========

    pub fn get_all_session_types(&self) -> SqlResult<Vec<SessionType>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, color, hourly_rate FROM session_types ORDER BY name")?;
        let types = stmt.query_map([], |row| {
            Ok(SessionType {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                hourly_rate: row.get(3)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        Ok(types)
    }

    pub fn add_session_type(&self, session_type: NewSessionType) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO session_types (name, color, hourly_rate) VALUES (?, ?, ?)",
            params![session_type.name, session_type.color, session_type.hourly_rate],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn update_session_type(&self, session_type: &SessionType) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE session_types SET name = ?, color = ?, hourly_rate = ? WHERE id = ?",
            params![session_type.name, session_type.color, session_type.hourly_rate, session_type.id],
        )?;
        Ok(())
    }

    pub fn delete_session_type(&self, id: i64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM session_types WHERE id = ?", params![id])?;
        Ok(())
    }

    // ========== SESSION OPERATIONS ==========

    pub fn get_all_sessions(&self) -> SqlResult<Vec<WorkSession>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT s.id, s.session_type_id, st.name, s.date, s.project_name, s.hours, s.description,
                    s.pay_type, s.hourly_rate, s.fixed_amount
             FROM sessions s
             LEFT JOIN session_types st ON s.session_type_id = st.id
             ORDER BY s.date DESC, s.id DESC"
        )?;

        let sessions = stmt.query_map([], |row| {
            let pay_type_str: Option<String> = row.get(7)?;
            Ok(WorkSession {
                id: row.get(0)?,
                session_type_id: row.get(1)?,
                session_type_name: row.get(2)?,
                date: row.get(3)?,
                project_name: row.get(4)?,
                hours: row.get(5)?,
                description: row.get(6)?,
                pay_type: pay_type_str.and_then(|s| PayType::from_string(&s).ok()),
                hourly_rate: row.get(8)?,
                fixed_amount: row.get(9)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        Ok(sessions)
    }

    pub fn get_sessions_by_date(&self, date: &str) -> SqlResult<Vec<WorkSession>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT s.id, s.session_type_id, st.name, s.date, s.project_name, s.hours, s.description,
                    s.pay_type, s.hourly_rate, s.fixed_amount
             FROM sessions s
             LEFT JOIN session_types st ON s.session_type_id = st.id
             WHERE s.date = ? ORDER BY s.id DESC"
        )?;

        let sessions = stmt.query_map([date], |row| {
            let pay_type_str: Option<String> = row.get(7)?;
            Ok(WorkSession {
                id: row.get(0)?,
                session_type_id: row.get(1)?,
                session_type_name: row.get(2)?,
                date: row.get(3)?,
                project_name: row.get(4)?,
                hours: row.get(5)?,
                description: row.get(6)?,
                pay_type: pay_type_str.and_then(|s| PayType::from_string(&s).ok()),
                hourly_rate: row.get(8)?,
                fixed_amount: row.get(9)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        Ok(sessions)
    }

    pub fn get_sessions_by_type_id(&self, session_type_id: i64) -> SqlResult<Vec<WorkSession>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT s.id, s.session_type_id, st.name, s.date, s.project_name, s.hours, s.description,
                    s.pay_type, s.hourly_rate, s.fixed_amount
             FROM sessions s
             LEFT JOIN session_types st ON s.session_type_id = st.id
             WHERE s.session_type_id = ? ORDER BY s.date DESC, s.id DESC"
        )?;

        let sessions = stmt.query_map([session_type_id], |row| {
            let pay_type_str: Option<String> = row.get(7)?;
            Ok(WorkSession {
                id: row.get(0)?,
                session_type_id: row.get(1)?,
                session_type_name: row.get(2)?,
                date: row.get(3)?,
                project_name: row.get(4)?,
                hours: row.get(5)?,
                description: row.get(6)?,
                pay_type: pay_type_str.and_then(|s| PayType::from_string(&s).ok()),
                hourly_rate: row.get(8)?,
                fixed_amount: row.get(9)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        Ok(sessions)
    }

    pub fn add_session(&self, session: NewSession) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO sessions (session_type_id, date, project_name, hours, description, pay_type, hourly_rate, fixed_amount)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                session.session_type_id,
                session.date,
                session.project_name,
                session.hours,
                session.description,
                session.pay_type.map(|t| t.to_string()),
                session.hourly_rate,
                session.fixed_amount,
            ],
        )?;
        let id = conn.last_insert_rowid();
        self.update_project_cache_internal(&conn, &session.project_name, session.session_type_id, &session.date)?;
        Ok(id)
    }

    pub fn update_session(&self, session: &WorkSession) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE sessions SET session_type_id = ?, date = ?, project_name = ?, hours = ?,
             description = ?, pay_type = ?, hourly_rate = ?, fixed_amount = ? WHERE id = ?",
            params![
                session.session_type_id,
                session.date,
                session.project_name,
                session.hours,
                session.description,
                session.pay_type.map(|t| t.to_string()),
                session.hourly_rate,
                session.fixed_amount,
                session.id,
            ],
        )?;
        self.update_project_cache_internal(&conn, &session.project_name, session.session_type_id, &session.date)?;
        Ok(())
    }

    pub fn delete_session(&self, id: i64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM sessions WHERE id = ?", params![id])?;
        Ok(())
    }

    // ========== PAY SUMMARY ==========

    pub fn get_pay_summary(&self) -> SqlResult<PaySummary> {
        let conn = self.conn.lock().unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let month_start = chrono::Local::now().format("%Y-%m-01").to_string();
        let year_start = chrono::Local::now().format("%Y-01-01").to_string();

        let calculate_pay = |sessions: &[WorkSession]| -> f64 {
            sessions.iter().map(|s| s.calculate_pay()).sum()
        };

        // Today's pay
        let today_sessions = self.get_sessions_by_date_internal(&conn, &today)?;
        let today_pay = calculate_pay(&today_sessions);

        // This month's pay
        let month_sessions = self.get_sessions_in_range_internal(&conn, &month_start, &today)?;
        let month_pay = calculate_pay(&month_sessions);

        // This year's pay
        let year_sessions = self.get_sessions_in_range_internal(&conn, &year_start, &today)?;
        let year_pay = calculate_pay(&year_sessions);

        // All time pay
        let all_sessions = self.get_all_sessions_internal(&conn)?;
        let all_time_pay = calculate_pay(&all_sessions);

        Ok(PaySummary {
            today: today_pay,
            this_month: month_pay,
            this_year: year_pay,
            all_time: all_time_pay,
        })
    }

    fn get_sessions_by_date_internal(&self, conn: &Connection, date: &str) -> SqlResult<Vec<WorkSession>> {
        let mut stmt = conn.prepare(
            "SELECT s.id, s.session_type_id, st.name, s.date, s.project_name, s.hours, s.description,
                    s.pay_type, s.hourly_rate, s.fixed_amount
             FROM sessions s
             LEFT JOIN session_types st ON s.session_type_id = st.id
             WHERE s.date = ?"
        )?;
        let sessions = stmt.query_map([date], |row| {
            let pay_type_str: Option<String> = row.get(7)?;
            Ok(WorkSession {
                id: row.get(0)?,
                session_type_id: row.get(1)?,
                session_type_name: row.get(2)?,
                date: row.get(3)?,
                project_name: row.get(4)?,
                hours: row.get(5)?,
                description: row.get(6)?,
                pay_type: pay_type_str.and_then(|s| PayType::from_string(&s).ok()),
                hourly_rate: row.get(8)?,
                fixed_amount: row.get(9)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        Ok(sessions)
    }

    fn get_sessions_in_range_internal(&self, conn: &Connection, start: &str, end: &str) -> SqlResult<Vec<WorkSession>> {
        let mut stmt = conn.prepare(
            "SELECT s.id, s.session_type_id, st.name, s.date, s.project_name, s.hours, s.description,
                    s.pay_type, s.hourly_rate, s.fixed_amount
             FROM sessions s
             LEFT JOIN session_types st ON s.session_type_id = st.id
             WHERE s.date >= ? AND s.date <= ?"
        )?;
        let sessions = stmt.query_map([start, end], |row| {
            let pay_type_str: Option<String> = row.get(7)?;
            Ok(WorkSession {
                id: row.get(0)?,
                session_type_id: row.get(1)?,
                session_type_name: row.get(2)?,
                date: row.get(3)?,
                project_name: row.get(4)?,
                hours: row.get(5)?,
                description: row.get(6)?,
                pay_type: pay_type_str.and_then(|s| PayType::from_string(&s).ok()),
                hourly_rate: row.get(8)?,
                fixed_amount: row.get(9)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        Ok(sessions)
    }

    fn get_all_sessions_internal(&self, conn: &Connection) -> SqlResult<Vec<WorkSession>> {
        let mut stmt = conn.prepare(
            "SELECT s.id, s.session_type_id, st.name, s.date, s.project_name, s.hours, s.description,
                    s.pay_type, s.hourly_rate, s.fixed_amount
             FROM sessions s
             LEFT JOIN session_types st ON s.session_type_id = st.id"
        )?;
        let sessions = stmt.query_map([], |row| {
            let pay_type_str: Option<String> = row.get(7)?;
            Ok(WorkSession {
                id: row.get(0)?,
                session_type_id: row.get(1)?,
                session_type_name: row.get(2)?,
                date: row.get(3)?,
                project_name: row.get(4)?,
                hours: row.get(5)?,
                description: row.get(6)?,
                pay_type: pay_type_str.and_then(|s| PayType::from_string(&s).ok()),
                hourly_rate: row.get(8)?,
                fixed_amount: row.get(9)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        Ok(sessions)
    }

    // ========== GOAL OPERATIONS ==========

    pub fn get_all_goals(&self) -> SqlResult<Vec<FinancialGoal>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, goal_type, name, target_amount, current_amount, created_date, target_date
             FROM goals ORDER BY created_date DESC"
        )?;

        let goals = stmt.query_map([], |row| {
            Ok(FinancialGoal {
                id: row.get(0)?,
                goal_type: GoalType::from_string(&row.get::<_, String>(1)?)
                    .map_err(|_| rusqlite::Error::InvalidQuery)?,
                name: row.get(2)?,
                target_amount: row.get(3)?,
                current_amount: row.get(4)?,
                created_date: row.get(5)?,
                target_date: row.get(6)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        Ok(goals)
    }

    pub fn add_goal(&self, goal: NewGoal) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO goals (goal_type, name, target_amount, current_amount, created_date, target_date)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![goal.goal_type.to_string(), goal.name, goal.target_amount, goal.current_amount, goal.created_date, goal.target_date],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn update_goal(&self, goal: &FinancialGoal) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE goals SET goal_type = ?, name = ?, target_amount = ?, current_amount = ?,
             created_date = ?, target_date = ? WHERE id = ?",
            params![goal.goal_type.to_string(), goal.name, goal.target_amount, goal.current_amount, goal.created_date, goal.target_date, goal.id],
        )?;
        Ok(())
    }

    pub fn add_contribution(&self, goal_id: i64, amount: f64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE goals SET current_amount = current_amount + ? WHERE id = ?", params![amount, goal_id])?;
        Ok(())
    }

    pub fn delete_goal(&self, id: i64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM goals WHERE id = ?", params![id])?;
        Ok(())
    }

    // ========== PROJECT CACHE ==========

    pub fn get_projects_by_type_id(&self, session_type_id: i64) -> SqlResult<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT DISTINCT project_name FROM projects WHERE session_type_id = ?
             ORDER BY use_count DESC, last_used DESC LIMIT 20"
        )?;
        let projects = stmt.query_map([session_type_id], |row| row.get(0))?.collect::<SqlResult<Vec<_>>>()?;
        Ok(projects)
    }

    fn update_project_cache_internal(&self, conn: &Connection, project_name: &str, session_type_id: i64, date: &str) -> SqlResult<()> {
        let updated = conn.execute(
            "UPDATE projects SET last_used = ?, use_count = use_count + 1 WHERE project_name = ? AND session_type_id = ?",
            params![date, project_name, session_type_id],
        )?;
        if updated == 0 {
            conn.execute(
                "INSERT INTO projects (project_name, session_type_id, last_used, use_count) VALUES (?, ?, ?, 1)",
                params![project_name, session_type_id, date],
            )?;
        }
        Ok(())
    }

    // ========== ANALYTICS ==========

    pub fn get_analytics(&self, range_start: &str, range_end: &str) -> SqlResult<AnalyticsData> {
        let conn = self.conn.lock().unwrap();

        // Get sessions in range with session type info
        let mut stmt = conn.prepare(
            "SELECT s.id, s.session_type_id, st.name, st.color, s.date, s.project_name, s.hours, 
                    s.description, s.pay_type, s.hourly_rate, s.fixed_amount
             FROM sessions s
             LEFT JOIN session_types st ON s.session_type_id = st.id
             WHERE s.date >= ? AND s.date <= ?
             ORDER BY s.date"
        )?;

        let sessions: Vec<(WorkSession, String)> = stmt.query_map([range_start, range_end], |row| {
            let pay_type_str: Option<String> = row.get(8)?;
            let color: String = row.get::<_, Option<String>>(3)?.unwrap_or("#6366F1".to_string());
            Ok((WorkSession {
                id: row.get(0)?,
                session_type_id: row.get(1)?,
                session_type_name: row.get(2)?,
                date: row.get(4)?,
                project_name: row.get(5)?,
                hours: row.get(6)?,
                description: row.get(7)?,
                pay_type: pay_type_str.and_then(|s| PayType::from_string(&s).ok()),
                hourly_rate: row.get(9)?,
                fixed_amount: row.get(10)?,
            }, color))
        })?.collect::<SqlResult<Vec<_>>>()?;

        // Summary calculations
        let total_hours: f64 = sessions.iter().map(|(s, _)| s.hours).sum();
        let total_sessions = sessions.len();
        let avg_session_length = if total_sessions > 0 { total_hours / total_sessions as f64 } else { 0.0 };
        let longest_session = sessions.iter().map(|(s, _)| s.hours).fold(0.0_f64, f64::max);
        let total_pay: f64 = sessions.iter().map(|(s, _)| s.calculate_pay()).sum();

        let summary = AnalyticsSummary {
            total_hours,
            total_sessions,
            avg_session_length,
            longest_session,
            total_pay,
        };

        // Daily hours aggregation
        let mut daily_map: std::collections::HashMap<String, (f64, f64)> = std::collections::HashMap::new();
        for (session, _) in &sessions {
            let entry = daily_map.entry(session.date.clone()).or_insert((0.0, 0.0));
            entry.0 += session.hours;
            entry.1 += session.calculate_pay();
        }
        let mut daily_hours: Vec<DailyHours> = daily_map.into_iter()
            .map(|(date, (hours, pay))| DailyHours { date, hours, pay })
            .collect();
        daily_hours.sort_by(|a, b| a.date.cmp(&b.date));

        // Category breakdown
        let mut category_map: std::collections::HashMap<String, (String, f64, usize, f64)> = std::collections::HashMap::new();
        for (session, color) in &sessions {
            let category_name = session.session_type_name.clone().unwrap_or("Unknown".to_string());
            let entry = category_map.entry(category_name).or_insert((color.clone(), 0.0, 0, 0.0));
            entry.1 += session.hours;
            entry.2 += 1;
            entry.3 += session.calculate_pay();
        }
        let category_breakdown: Vec<CategoryBreakdown> = category_map.into_iter()
            .map(|(category, (color, hours, sessions, pay))| CategoryBreakdown {
                category,
                color,
                hours,
                sessions,
                pay,
            })
            .collect();

        // Weekday breakdown
        let mut weekday_counts: [f64; 7] = [0.0; 7];
        let mut weekday_sessions: [usize; 7] = [0; 7];
        for (session, _) in &sessions {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(&session.date, "%Y-%m-%d") {
                let weekday = date.weekday().num_days_from_sunday() as usize;
                weekday_counts[weekday] += session.hours;
                weekday_sessions[weekday] += 1;
            }
        }
        let weekday_names = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        let weekday_breakdown: Vec<WeekdayBreakdown> = weekday_names.iter().enumerate()
            .map(|(i, name)| WeekdayBreakdown {
                weekday: name.to_string(),
                hours: weekday_counts[i],
                sessions: weekday_sessions[i],
            })
            .collect();

        Ok(AnalyticsData {
            summary,
            daily_hours,
            category_breakdown,
            weekday_breakdown,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PaySummary {
    pub today: f64,
    pub this_month: f64,
    pub this_year: f64,
    pub all_time: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AnalyticsSummary {
    pub total_hours: f64,
    pub total_sessions: usize,
    pub avg_session_length: f64,
    pub longest_session: f64,
    pub total_pay: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DailyHours {
    pub date: String,
    pub hours: f64,
    pub pay: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CategoryBreakdown {
    pub category: String,
    pub color: String,
    pub hours: f64,
    pub sessions: usize,
    pub pay: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WeekdayBreakdown {
    pub weekday: String,
    pub hours: f64,
    pub sessions: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AnalyticsData {
    pub summary: AnalyticsSummary,
    pub daily_hours: Vec<DailyHours>,
    pub category_breakdown: Vec<CategoryBreakdown>,
    pub weekday_breakdown: Vec<WeekdayBreakdown>,
}

// ========== ACHIEVEMENT OPERATIONS ==========

impl Database {
    /// Get all unlocked achievement IDs
    pub fn get_unlocked_achievements(&self) -> SqlResult<Vec<(String, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, unlocked_at FROM achievements")?;
        let achievements = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?.collect::<SqlResult<Vec<_>>>()?;
        Ok(achievements)
    }

    /// Unlock an achievement
    pub fn unlock_achievement(&self, achievement_id: &str) -> SqlResult<bool> {
        let conn = self.conn.lock().unwrap();
        let result = conn.execute(
            "INSERT OR IGNORE INTO achievements (id) VALUES (?)",
            params![achievement_id],
        )?;
        Ok(result > 0) // Returns true if newly inserted
    }

    /// Log an app event (for achievement tracking)
    pub fn log_app_event(&self, event_type: &str, event_data: Option<&str>) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO app_events (event_type, event_data) VALUES (?, ?)",
            params![event_type, event_data],
        )?;
        Ok(())
    }

    /// Count distinct days with events of a certain type
    pub fn count_event_days(&self, event_type: &str) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT date(created_at)) FROM app_events WHERE event_type = ?",
            params![event_type],
            |row| row.get(0)
        )?;
        Ok(count)
    }

    /// Count distinct event data values for a type
    pub fn count_distinct_event_data(&self, event_type: &str) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT event_data) FROM app_events WHERE event_type = ? AND event_data IS NOT NULL",
            params![event_type],
            |row| row.get(0)
        )?;
        Ok(count)
    }

    /// Check if two event types occurred on the same day
    pub fn events_same_day(&self, event1: &str, event2: &str) -> SqlResult<bool> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM (
                SELECT DISTINCT date(created_at) as d FROM app_events WHERE event_type = ?
            ) a JOIN (
                SELECT DISTINCT date(created_at) as d FROM app_events WHERE event_type = ?
            ) b ON a.d = b.d",
            params![event1, event2],
            |row| row.get(0)
        )?;
        Ok(count > 0)
    }

    // ========== ACHIEVEMENT CHECK QUERIES ==========

    /// Count total sessions
    pub fn count_total_sessions(&self) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |row| row.get(0))?;
        Ok(count)
    }

    /// Count distinct days with sessions
    pub fn count_distinct_session_days(&self) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT date) FROM sessions",
            [],
            |row| row.get(0)
        )?;
        Ok(count)
    }

    /// Count distinct weeks with sessions
    pub fn count_distinct_session_weeks(&self) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT strftime('%Y-%W', date)) FROM sessions",
            [],
            |row| row.get(0)
        )?;
        Ok(count)
    }

    /// Get total hours tracked
    pub fn get_total_hours(&self) -> SqlResult<f64> {
        let conn = self.conn.lock().unwrap();
        let hours: f64 = conn.query_row(
            "SELECT COALESCE(SUM(hours), 0) FROM sessions",
            [],
            |row| row.get(0)
        )?;
        Ok(hours)
    }

    /// Check if user has any paid sessions
    pub fn has_paid_session(&self) -> SqlResult<bool> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sessions WHERE pay_type IN ('Hourly', 'Fixed') AND (hourly_rate > 0 OR fixed_amount > 0)",
            [],
            |row| row.get(0)
        )?;
        Ok(count > 0)
    }

    /// Check for sustainable week (avg < 8h/day for 7 consecutive days with sessions)
    pub fn has_sustainable_week(&self) -> SqlResult<bool> {
        let conn = self.conn.lock().unwrap();
        // Get daily totals ordered by date
        let mut stmt = conn.prepare(
            "SELECT date, SUM(hours) as daily_hours FROM sessions 
             GROUP BY date ORDER BY date"
        )?;
        let days: Vec<(String, f64)> = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })?.collect::<SqlResult<Vec<_>>>()?;
        
        if days.len() < 7 {
            return Ok(false);
        }
        
        // Check any 7-day window
        for window in days.windows(7) {
            let total: f64 = window.iter().map(|(_, h)| h).sum();
            if total / 7.0 < 8.0 {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Check for paced week (no session > 6h in past 7 days with sessions)
    pub fn has_paced_week(&self) -> SqlResult<bool> {
        let conn = self.conn.lock().unwrap();
        // Need at least 7 distinct days of sessions
        let day_count: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT date) FROM sessions",
            [],
            |row| row.get(0)
        )?;
        
        if day_count < 7 {
            return Ok(false);
        }
        
        // Check if there's a window of 7 days with no session > 6h
        let max_in_last_week: f64 = conn.query_row(
            "WITH recent_days AS (
                SELECT DISTINCT date FROM sessions ORDER BY date DESC LIMIT 7
            )
            SELECT COALESCE(MAX(hours), 0) FROM sessions 
            WHERE date IN (SELECT date FROM recent_days)",
            [],
            |row| row.get(0)
        )?;
        
        Ok(max_in_last_week <= 6.0)
    }

    /// Check for human weekend (< 3h total on a Sat+Sun)
    pub fn has_human_weekend(&self) -> SqlResult<bool> {
        let conn = self.conn.lock().unwrap();
        // Find weekends (Sat=6, Sun=0 in strftime)
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM (
                SELECT strftime('%Y-%W', date) as week, SUM(hours) as weekend_hours
                FROM sessions
                WHERE strftime('%w', date) IN ('0', '6')
                GROUP BY week
                HAVING weekend_hours < 3
            )",
            [],
            |row| row.get(0)
        )?;
        Ok(count > 0)
    }

    // ========== INVOICE OPERATIONS ==========

    pub fn get_all_invoices(&self) -> SqlResult<Vec<Invoice>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, invoice_number, client_name, client_email, created_date, due_date, 
                    status, subtotal, tax_rate, tax_amount, total, notes
             FROM invoices ORDER BY created_date DESC"
        )?;
        
        let invoices: Vec<Invoice> = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let status_str: String = row.get(6)?;
            Ok(Invoice {
                id,
                invoice_number: row.get(1)?,
                client_name: row.get(2)?,
                client_email: row.get(3)?,
                created_date: row.get(4)?,
                due_date: row.get(5)?,
                status: InvoiceStatus::from_string(&status_str).unwrap_or(InvoiceStatus::Draft),
                subtotal: row.get(7)?,
                tax_rate: row.get(8)?,
                tax_amount: row.get(9)?,
                total: row.get(10)?,
                notes: row.get(11)?,
                items: Vec::new(), // Will be populated below
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        
        // Get items for each invoice
        let mut result = invoices;
        for invoice in &mut result {
            invoice.items = self.get_invoice_items_internal(&conn, invoice.id)?;
        }
        
        Ok(result)
    }

    pub fn get_invoice(&self, id: i64) -> SqlResult<Invoice> {
        let conn = self.conn.lock().unwrap();
        let mut invoice: Invoice = conn.query_row(
            "SELECT id, invoice_number, client_name, client_email, created_date, due_date, 
                    status, subtotal, tax_rate, tax_amount, total, notes
             FROM invoices WHERE id = ?",
            params![id],
            |row| {
                let status_str: String = row.get(6)?;
                Ok(Invoice {
                    id: row.get(0)?,
                    invoice_number: row.get(1)?,
                    client_name: row.get(2)?,
                    client_email: row.get(3)?,
                    created_date: row.get(4)?,
                    due_date: row.get(5)?,
                    status: InvoiceStatus::from_string(&status_str).unwrap_or(InvoiceStatus::Draft),
                    subtotal: row.get(7)?,
                    tax_rate: row.get(8)?,
                    tax_amount: row.get(9)?,
                    total: row.get(10)?,
                    notes: row.get(11)?,
                    items: Vec::new(),
                })
            }
        )?;
        
        invoice.items = self.get_invoice_items_internal(&conn, id)?;
        Ok(invoice)
    }

    fn get_invoice_items_internal(&self, conn: &Connection, invoice_id: i64) -> SqlResult<Vec<InvoiceItem>> {
        let mut stmt = conn.prepare(
            "SELECT id, invoice_id, session_id, description, hours, rate, amount
             FROM invoice_items WHERE invoice_id = ?"
        )?;
        
        let items = stmt.query_map(params![invoice_id], |row| {
            Ok(InvoiceItem {
                id: row.get(0)?,
                invoice_id: row.get(1)?,
                session_id: row.get(2)?,
                description: row.get(3)?,
                hours: row.get(4)?,
                rate: row.get(5)?,
                amount: row.get(6)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        
        Ok(items)
    }

    pub fn create_invoice(&self, invoice: NewInvoice) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        
        // Generate invoice number
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM invoices", [], |row| row.get(0))?;
        let invoice_number = format!("INV-{:04}", count + 1);
        let created_date = chrono::Local::now().format("%Y-%m-%d").to_string();
        
        // Get sessions to calculate totals
        let session_ids_str = invoice.session_ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        
        let sessions: Vec<(i64, String, f64, Option<f64>, Option<f64>, Option<String>, Option<String>)> = if !invoice.session_ids.is_empty() {
            let mut stmt = conn.prepare(&format!(
                "SELECT s.id, s.project_name, s.hours, s.hourly_rate, s.fixed_amount, s.pay_type, st.name
                 FROM sessions s
                 LEFT JOIN session_types st ON s.session_type_id = st.id
                 WHERE s.id IN ({})", session_ids_str
            ))?;
            let result: Vec<_> = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, f64>(2)?,
                    row.get::<_, Option<f64>>(3)?,
                    row.get::<_, Option<f64>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                    row.get::<_, Option<String>>(6)?,
                ))
            })?.collect::<SqlResult<Vec<_>>>()?;
            result
        } else {
            Vec::new()
        };
        
        // Calculate subtotal
        let subtotal: f64 = sessions.iter().map(|(_, _, hours, hourly_rate, fixed_amount, pay_type, _)| {
            match pay_type.as_deref() {
                Some("Hourly") => hourly_rate.unwrap_or(0.0) * hours,
                Some("Fixed") => fixed_amount.unwrap_or(0.0),
                _ => 0.0,
            }
        }).sum();
        
        let tax_amount = subtotal * invoice.tax_rate.unwrap_or(0.0) / 100.0;
        let total = subtotal + tax_amount;
        
        // Insert invoice
        conn.execute(
            "INSERT INTO invoices (invoice_number, client_name, client_email, created_date, due_date, 
                                   status, subtotal, tax_rate, tax_amount, total, notes)
             VALUES (?, ?, ?, ?, ?, 'Draft', ?, ?, ?, ?, ?)",
            params![
                invoice_number,
                invoice.client_name,
                invoice.client_email,
                created_date,
                invoice.due_date,
                subtotal,
                invoice.tax_rate,
                tax_amount,
                total,
                invoice.notes
            ],
        )?;
        
        let invoice_id = conn.last_insert_rowid();
        
        // Insert line items
        for (session_id, project_name, hours, hourly_rate, fixed_amount, pay_type, session_type) in sessions {
            let rate = hourly_rate.unwrap_or(0.0);
            let amount = match pay_type.as_deref() {
                Some("Hourly") => rate * hours,
                Some("Fixed") => fixed_amount.unwrap_or(0.0),
                _ => 0.0,
            };
            let description = format!("{} - {}", session_type.unwrap_or("Work".to_string()), project_name);
            
            conn.execute(
                "INSERT INTO invoice_items (invoice_id, session_id, description, hours, rate, amount)
                 VALUES (?, ?, ?, ?, ?, ?)",
                params![invoice_id, session_id, description, hours, rate, amount],
            )?;
        }
        
        Ok(invoice_id)
    }

    pub fn update_invoice_status(&self, id: i64, status: InvoiceStatus) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE invoices SET status = ? WHERE id = ?",
            params![status.to_string(), id],
        )?;
        Ok(())
    }

    pub fn delete_invoice(&self, id: i64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM invoice_items WHERE invoice_id = ?", params![id])?;
        conn.execute("DELETE FROM invoices WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn get_uninvoiced_sessions(&self) -> SqlResult<Vec<WorkSession>> {
        let conn = self.conn.lock().unwrap();
        // Get all sessions that haven't been invoiced yet
        let mut stmt = conn.prepare(
            "SELECT s.id, s.session_type_id, st.name as session_type_name, s.date, 
                    COALESCE(s.project_name, 'Unnamed Session') as project_name, 
                    s.hours, s.description,
                    s.pay_type, 
                    COALESCE(s.hourly_rate, st.hourly_rate) as hourly_rate, 
                    s.fixed_amount
             FROM sessions s
             LEFT JOIN session_types st ON s.session_type_id = st.id
             WHERE s.id NOT IN (SELECT DISTINCT session_id FROM invoice_items WHERE session_id IS NOT NULL)
             ORDER BY s.date DESC"
        )?;
        
        let sessions = stmt.query_map([], |row| {
            let pay_type_str: Option<String> = row.get(7)?;
            Ok(WorkSession {
                id: row.get(0)?,
                session_type_id: row.get(1)?,
                session_type_name: row.get(2)?,
                date: row.get(3)?,
                project_name: row.get(4)?,
                hours: row.get(5)?,
                description: row.get(6)?,
                pay_type: pay_type_str.and_then(|s| PayType::from_string(&s).ok()),
                hourly_rate: row.get(8)?,
                fixed_amount: row.get(9)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        
        Ok(sessions)
    }

    // ========== HABIT OPERATIONS ==========

    pub fn get_all_habits(&self) -> SqlResult<Vec<Habit>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT h.id, h.name, h.description, h.trigger_type, h.trigger_value, h.reward_description, h.is_active,
                    COALESCE((SELECT COUNT(*) FROM habit_logs WHERE habit_id = h.id), 0) as total_completions
             FROM habits h ORDER BY h.name"
        )?;
        
        let habits: Vec<Habit> = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let trigger_type: String = row.get(3)?;
            let trigger_value: f64 = row.get(4)?;
            let total_completions: i32 = row.get(7)?;
            
            // Convert trigger_value to minutes if it's an hours-based trigger
            let trigger_minutes = if trigger_type == "after_hours" {
                Some((trigger_value * 60.0) as i32)
            } else {
                None
            };
            
            Ok(Habit {
                id,
                name: row.get(1)?,
                description: row.get(2)?,
                trigger_type,
                trigger_value,
                reward_description: row.get(5)?,
                is_active: row.get::<_, i32>(6)? == 1,
                current_streak: 0, // Will be computed below
                best_streak: 0,    // Will be computed below
                total_completions,
                target_streak: 7,  // Default target
                trigger_minutes,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        
        // Compute streaks for each habit
        let mut result = habits;
        for habit in &mut result {
            let (current, best) = self.compute_habit_streaks(&conn, habit.id)?;
            habit.current_streak = current;
            habit.best_streak = best;
        }
        
        Ok(result)
    }
    
    fn compute_habit_streaks(&self, conn: &Connection, habit_id: i64) -> SqlResult<(i32, i32)> {
        // Get all completion dates for this habit, ordered by date
        let mut stmt = conn.prepare(
            "SELECT DISTINCT date(completed_at) as completion_date 
             FROM habit_logs WHERE habit_id = ? 
             ORDER BY completion_date DESC"
        )?;
        
        let dates: Vec<String> = stmt.query_map(params![habit_id], |row| {
            row.get(0)
        })?.collect::<SqlResult<Vec<_>>>()?;
        
        if dates.is_empty() {
            return Ok((0, 0));
        }
        
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let yesterday = (chrono::Local::now() - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
        
        // Calculate current streak
        let mut current_streak = 0;
        let mut check_date = chrono::Local::now().date_naive();
        
        // Start from today or yesterday
        if dates.contains(&today) || dates.contains(&yesterday) {
            for date_str in &dates {
                let check_str = check_date.format("%Y-%m-%d").to_string();
                if date_str == &check_str {
                    current_streak += 1;
                    check_date = check_date.pred_opt().unwrap_or(check_date);
                } else if date_str < &check_str {
                    // Check if this date is the next consecutive day
                    let prev_str = check_date.pred_opt().unwrap_or(check_date).format("%Y-%m-%d").to_string();
                    if date_str == &prev_str {
                        current_streak += 1;
                        check_date = check_date.pred_opt().unwrap_or(check_date).pred_opt().unwrap_or(check_date);
                    } else {
                        break;
                    }
                }
            }
        }
        
        // Calculate best streak
        let mut best_streak = 0;
        let mut streak = 1;
        let mut sorted_dates = dates.clone();
        sorted_dates.sort();
        
        for i in 1..sorted_dates.len() {
            if let (Ok(prev), Ok(curr)) = (
                chrono::NaiveDate::parse_from_str(&sorted_dates[i-1], "%Y-%m-%d"),
                chrono::NaiveDate::parse_from_str(&sorted_dates[i], "%Y-%m-%d"),
            ) {
                if (curr - prev).num_days() == 1 {
                    streak += 1;
                } else {
                    best_streak = best_streak.max(streak);
                    streak = 1;
                }
            }
        }
        best_streak = best_streak.max(streak);
        
        Ok((current_streak, best_streak))
    }

    pub fn add_habit(&self, habit: Habit) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO habits (name, description, trigger_type, trigger_value, reward_description, is_active)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                habit.name,
                habit.description,
                habit.trigger_type,
                habit.trigger_value,
                habit.reward_description,
                if habit.is_active { 1 } else { 0 }
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn update_habit(&self, habit: &Habit) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE habits SET name = ?, description = ?, trigger_type = ?, trigger_value = ?, 
             reward_description = ?, is_active = ? WHERE id = ?",
            params![
                habit.name,
                habit.description,
                habit.trigger_type,
                habit.trigger_value,
                habit.reward_description,
                if habit.is_active { 1 } else { 0 },
                habit.id
            ],
        )?;
        Ok(())
    }

    pub fn delete_habit(&self, id: i64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM habit_logs WHERE habit_id = ?", params![id])?;
        conn.execute("DELETE FROM habits WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn log_habit_completion(&self, habit_id: i64, notes: Option<String>) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO habit_logs (habit_id, notes) VALUES (?, ?)",
            params![habit_id, notes],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_habit_logs_for_date(&self, date: &str) -> SqlResult<Vec<HabitLog>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, habit_id, completed_at, notes
             FROM habit_logs WHERE date(completed_at) = ?"
        )?;
        
        let logs = stmt.query_map(params![date], |row| {
            Ok(HabitLog {
                id: row.get(0)?,
                habit_id: row.get(1)?,
                completed_at: row.get(2)?,
                notes: row.get(3)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        
        Ok(logs)
    }

    // ========== AUTO-TRACKING OPERATIONS ==========

    pub fn get_pending_suggestions(&self) -> SqlResult<Vec<ActivitySuggestion>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, date, app_name, window_title, suggested_project, suggested_session_type_id,
                    duration_minutes, start_time, end_time, status
             FROM activity_suggestions WHERE status = 'pending' ORDER BY date DESC, start_time DESC"
        )?;
        
        let suggestions = stmt.query_map([], |row| {
            Ok(ActivitySuggestion {
                id: row.get(0)?,
                date: row.get(1)?,
                app_name: row.get(2)?,
                window_title: row.get(3)?,
                suggested_project: row.get(4)?,
                suggested_session_type_id: row.get(5)?,
                duration_minutes: row.get(6)?,
                start_time: row.get(7)?,
                end_time: row.get(8)?,
                status: row.get(9)?,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        
        Ok(suggestions)
    }

    pub fn accept_suggestion(&self, suggestion_id: i64, session_type_id: i64, project_name: &str) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        
        // Get suggestion
        let suggestion: ActivitySuggestion = conn.query_row(
            "SELECT id, date, app_name, window_title, suggested_project, suggested_session_type_id,
                    duration_minutes, start_time, end_time, status
             FROM activity_suggestions WHERE id = ?",
            params![suggestion_id],
            |row| {
                Ok(ActivitySuggestion {
                    id: row.get(0)?,
                    date: row.get(1)?,
                    app_name: row.get(2)?,
                    window_title: row.get(3)?,
                    suggested_project: row.get(4)?,
                    suggested_session_type_id: row.get(5)?,
                    duration_minutes: row.get(6)?,
                    start_time: row.get(7)?,
                    end_time: row.get(8)?,
                    status: row.get(9)?,
                })
            }
        )?;
        
        // Create session from suggestion
        let hours = suggestion.duration_minutes / 60.0;
        conn.execute(
            "INSERT INTO sessions (session_type_id, date, project_name, hours, description, pay_type)
             VALUES (?, ?, ?, ?, ?, 'None')",
            params![
                session_type_id,
                suggestion.date,
                project_name,
                hours,
                format!("Auto-tracked: {} - {}", suggestion.app_name, suggestion.window_title)
            ],
        )?;
        
        let session_id = conn.last_insert_rowid();
        
        // Update suggestion status
        conn.execute(
            "UPDATE activity_suggestions SET status = 'accepted' WHERE id = ?",
            params![suggestion_id],
        )?;
        
        Ok(session_id)
    }

    pub fn dismiss_suggestion(&self, suggestion_id: i64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE activity_suggestions SET status = 'dismissed' WHERE id = ?",
            params![suggestion_id],
        )?;
        Ok(())
    }

    pub fn get_tracking_rules(&self) -> SqlResult<Vec<TrackingRule>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, app_pattern, title_pattern, project_name, session_type_id, is_active
             FROM tracking_rules ORDER BY app_pattern"
        )?;
        
        let rules = stmt.query_map([], |row| {
            Ok(TrackingRule {
                id: row.get(0)?,
                app_pattern: row.get(1)?,
                title_pattern: row.get(2)?,
                project_name: row.get(3)?,
                session_type_id: row.get(4)?,
                is_active: row.get::<_, i32>(5)? == 1,
            })
        })?.collect::<SqlResult<Vec<_>>>()?;
        
        Ok(rules)
    }

    pub fn add_tracking_rule(&self, rule: TrackingRule) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tracking_rules (app_pattern, title_pattern, project_name, session_type_id, is_active)
             VALUES (?, ?, ?, ?, ?)",
            params![
                rule.app_pattern,
                rule.title_pattern,
                rule.project_name,
                rule.session_type_id,
                if rule.is_active { 1 } else { 0 }
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn delete_tracking_rule(&self, id: i64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM tracking_rules WHERE id = ?", params![id])?;
        Ok(())
    }

    // ========== LICENSE ==========

    pub fn get_license(&self) -> SqlResult<License> {
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            "SELECT tier, license_key, activated_at, expires_at FROM license WHERE id = 1",
            [],
            |row| {
                let tier_str: String = row.get(0)?;
                Ok(License {
                    tier: Tier::from_string(&tier_str),
                    license_key: row.get(1)?,
                    activated_at: row.get(2)?,
                    expires_at: row.get(3)?,
                })
            }
        );

        match result {
            Ok(license) => Ok(license),
            Err(_) => Ok(License::default()),
        }
    }

    pub fn save_license(&self, license: &License) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO license (id, tier, license_key, activated_at, expires_at, updated_at)
             VALUES (1, ?, ?, ?, ?, datetime('now'))",
            params![
                license.tier.to_string(),
                license.license_key,
                license.activated_at,
                license.expires_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_session_type_count(&self) -> SqlResult<u32> {
        let conn = self.conn.lock().unwrap();
        let count: u32 = conn.query_row(
            "SELECT COUNT(*) FROM session_types",
            [],
            |row| row.get(0)
        )?;
        Ok(count)
    }

    pub fn get_goal_count(&self) -> SqlResult<u32> {
        let conn = self.conn.lock().unwrap();
        let count: u32 = conn.query_row(
            "SELECT COUNT(*) FROM goals",
            [],
            |row| row.get(0)
        )?;
        Ok(count)
    }

    // ========== SETTINGS ==========

    pub fn get_setting(&self, key: &str) -> SqlResult<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            "SELECT value FROM settings WHERE key = ?",
            params![key],
            |row| row.get(0)
        );
        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn delete_setting(&self, key: &str) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM settings WHERE key = ?", params![key])?;
        Ok(())
    }
}
