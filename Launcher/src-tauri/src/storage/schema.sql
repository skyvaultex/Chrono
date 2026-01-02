-- Session Types table (user-defined)
CREATE TABLE IF NOT EXISTS session_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL DEFAULT '#6366F1',
    hourly_rate REAL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_type_id INTEGER NOT NULL,
    date TEXT NOT NULL,
    project_name TEXT NOT NULL,
    hours REAL NOT NULL CHECK(hours >= 0.1 AND hours <= 24.0),
    description TEXT,
    pay_type TEXT CHECK(pay_type IN ('None', 'Hourly', 'Fixed')),
    hourly_rate REAL,
    fixed_amount REAL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (session_type_id) REFERENCES session_types(id)
);

CREATE INDEX IF NOT EXISTS idx_sessions_date ON sessions(date);
CREATE INDEX IF NOT EXISTS idx_sessions_type_date ON sessions(session_type_id, date);
CREATE INDEX IF NOT EXISTS idx_sessions_project ON sessions(project_name);

-- Goals table
CREATE TABLE IF NOT EXISTS goals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    goal_type TEXT NOT NULL CHECK(goal_type IN ('Debt', 'Purchase', 'Savings')),
    name TEXT NOT NULL,
    target_amount REAL NOT NULL CHECK(target_amount > 0),
    current_amount REAL NOT NULL DEFAULT 0 CHECK(current_amount >= 0),
    created_date TEXT NOT NULL,
    target_date TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_goals_type ON goals(goal_type);
CREATE INDEX IF NOT EXISTS idx_goals_created ON goals(created_date);

-- Projects cache table (for smart selector)
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_name TEXT NOT NULL,
    session_type_id INTEGER NOT NULL,
    last_used TEXT NOT NULL,
    use_count INTEGER NOT NULL DEFAULT 1,
    UNIQUE(project_name, session_type_id),
    FOREIGN KEY (session_type_id) REFERENCES session_types(id)
);

CREATE INDEX IF NOT EXISTS idx_projects_type_name ON projects(session_type_id, project_name);
CREATE INDEX IF NOT EXISTS idx_projects_last_used ON projects(last_used DESC);
CREATE INDEX IF NOT EXISTS idx_projects_use_count ON projects(use_count DESC);

-- Achievements table (tracks unlocked achievements)
CREATE TABLE IF NOT EXISTS achievements (
    id TEXT PRIMARY KEY,
    unlocked_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- App events table (tracks specific actions for achievement logic)
CREATE TABLE IF NOT EXISTS app_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL,
    event_data TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_app_events_type ON app_events(event_type);
CREATE INDEX IF NOT EXISTS idx_app_events_date ON app_events(created_at);

-- ========== INVOICING TABLES ==========

-- Invoices table
CREATE TABLE IF NOT EXISTS invoices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_number TEXT NOT NULL UNIQUE,
    client_name TEXT NOT NULL,
    client_email TEXT,
    created_date TEXT NOT NULL DEFAULT (date('now')),
    due_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft' CHECK(status IN ('Draft', 'Sent', 'Paid', 'Overdue')),
    subtotal REAL NOT NULL DEFAULT 0,
    tax_rate REAL,
    tax_amount REAL NOT NULL DEFAULT 0,
    total REAL NOT NULL DEFAULT 0,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
CREATE INDEX IF NOT EXISTS idx_invoices_client ON invoices(client_name);
CREATE INDEX IF NOT EXISTS idx_invoices_date ON invoices(created_date);

-- Invoice items table
CREATE TABLE IF NOT EXISTS invoice_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_id INTEGER NOT NULL,
    session_id INTEGER,
    description TEXT NOT NULL,
    hours REAL NOT NULL DEFAULT 0,
    rate REAL NOT NULL DEFAULT 0,
    amount REAL NOT NULL DEFAULT 0,
    FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_invoice_items_invoice ON invoice_items(invoice_id);

-- ========== AUTO-TRACKING TABLES ==========

-- Activity suggestions from auto-tracking
CREATE TABLE IF NOT EXISTS activity_suggestions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,
    app_name TEXT NOT NULL,
    window_title TEXT,
    suggested_project TEXT,
    suggested_session_type_id INTEGER,
    duration_minutes REAL NOT NULL DEFAULT 0,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'accepted', 'dismissed')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (suggested_session_type_id) REFERENCES session_types(id)
);

CREATE INDEX IF NOT EXISTS idx_activity_suggestions_status ON activity_suggestions(status);
CREATE INDEX IF NOT EXISTS idx_activity_suggestions_date ON activity_suggestions(date);

-- Tracking rules for auto-classification
CREATE TABLE IF NOT EXISTS tracking_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    app_pattern TEXT NOT NULL,
    title_pattern TEXT,
    project_name TEXT NOT NULL,
    session_type_id INTEGER NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (session_type_id) REFERENCES session_types(id)
);

CREATE INDEX IF NOT EXISTS idx_tracking_rules_active ON tracking_rules(is_active);

-- ========== HABITS TABLES ==========

-- Habits definition
CREATE TABLE IF NOT EXISTS habits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    trigger_type TEXT NOT NULL CHECK(trigger_type IN ('after_session', 'after_hours', 'daily')),
    trigger_value REAL NOT NULL DEFAULT 1,
    reward_description TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Habit completion logs
CREATE TABLE IF NOT EXISTS habit_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    habit_id INTEGER NOT NULL,
    completed_at TEXT NOT NULL DEFAULT (datetime('now')),
    notes TEXT,
    FOREIGN KEY (habit_id) REFERENCES habits(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_habit_logs_habit ON habit_logs(habit_id);
CREATE INDEX IF NOT EXISTS idx_habit_logs_date ON habit_logs(completed_at);

-- ========== LICENSE TABLE ==========

-- License storage (local, offline-first)
CREATE TABLE IF NOT EXISTS license (
    id INTEGER PRIMARY KEY CHECK (id = 1),  -- Only one row allowed
    tier TEXT NOT NULL DEFAULT 'Free' CHECK(tier IN ('Free', 'Pro', 'Lifetime')),
    license_key TEXT,
    activated_at TEXT,
    expires_at TEXT,  -- NULL for Lifetime, date for Pro subscriptions
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Insert default free tier if not exists
INSERT OR IGNORE INTO license (id, tier) VALUES (1, 'Free');

-- ========== SETTINGS TABLE ==========

-- Key-value store for app settings
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
