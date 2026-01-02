# Chrono ⏱️

A modern desktop work session tracker and financial goal manager built with Tauri, Rust, Svelte, and TypeScript.

## Features

### 📊 Work Session Tracking
- **Two Session Types**:
  - **Construction**: Automatic pay calculation at $30/hour
  - **Coding**: Track programming language and project details
- Smart project name autocomplete
- Date tracking with today's date as default
- Hours validation (0.1 - 24.0)

### 💰 Financial Goals
- Track multiple goal types:
  - Debt Payoff
  - Purchase Goals
  - Savings Targets
- Progress bars and percentage tracking
- Automatic ETA calculation based on average weekly income
- Quick contribution updates

### 🎯 Dashboard
- Today's work summary
- Construction vs Coding hours breakdown
- Daily pay calculations
- Recent sessions overview

### 🤖 AI Advisor (Coming Soon)
- ChatGPT integration for personalized financial advice
- Payoff strategy recommendations
- Budget optimization
- Work-life balance suggestions

## Tech Stack

### Backend
- **Tauri** - Lightweight desktop framework
- **Rust** - Safe, fast system programming
- **SQLite** (via rusqlite) - Local database storage

### Frontend
- **Svelte** + **TypeScript** - Reactive UI framework
- **Tailwind CSS** - Utility-first styling
- **Skeleton UI** - Svelte component library

## Project Structure

```
chrono/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── core/        # Domain logic
│   │   │   ├── models.rs    # Data models
│   │   │   └── logic.rs     # Business logic
│   │   ├── storage/     # Database layer
│   │   │   ├── db.rs        # SQLite operations
│   │   │   └── schema.sql   # Database schema
│   │   ├── commands.rs  # Tauri API commands
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                 # Svelte frontend
│   ├── lib/
│   │   ├── components/  # UI components
│   │   ├── api.ts       # Tauri command wrappers
│   │   └── types.ts     # TypeScript interfaces
│   ├── App.svelte       # Main app component
│   └── main.ts
├── package.json
├── vite.config.ts
└── tailwind.config.ts
```

## Getting Started

### Prerequisites
- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

1. **Install frontend dependencies**:
   ```powershell
   npm install
   ```

2. **Run in development mode**:
   ```powershell
   npm run tauri dev
   ```

3. **Build for production**:
   ```powershell
   npm run tauri build
   ```

## Database Schema

### Sessions Table
- `id` - Primary key
- `session_type` - 'Construction' or 'Coding'
- `date` - YYYY-MM-DD format
- `project_name` - Project identifier
- `hours` - Hours worked (0.1-24.0)
- `language` - Programming language (Coding only)
- `description` - Session details (Coding only)

### Goals Table
- `id` - Primary key
- `goal_type` - 'Debt', 'Purchase', or 'Savings'
- `name` - Goal description
- `target_amount` - Target dollar amount
- `current_amount` - Current progress
- `created_date` - When goal was created
- `target_date` - Optional completion target

### Projects Cache
- Remembers previously used project names
- Enables smart autocomplete
- Tracks usage frequency

## Development

### Backend (Rust)
The Rust backend is organized into modules:
- **core/models.rs**: Data structures and validation
- **core/logic.rs**: Business logic (calculations, ETA, etc.)
- **storage/db.rs**: Database operations
- **commands.rs**: Tauri command handlers

### Frontend (Svelte)
Component structure:
- **Dashboard.svelte**: Overview and summary cards
- **Sessions.svelte**: Session list with tabs
- **SessionForm.svelte**: Add/edit session dialog
- **Goals.svelte**: Financial goals management
- **GoalForm.svelte**: Add/edit goal dialog
- **ContributionDialog.svelte**: Quick contribution input
- **Advisor.svelte**: AI advisor interface (stub)

### API Layer
All Tauri commands are wrapped in `src/lib/api.ts` with proper TypeScript types.

## Future Enhancements

- [ ] OpenAI ChatGPT integration for AI Advisor
- [ ] Export data to CSV/PDF
- [ ] Charts and analytics
- [ ] Multi-user support
- [ ] Cloud sync (optional)
- [ ] Mobile app companion
- [ ] Recurring goal contributions
- [ ] Session timer/stopwatch
- [ ] Dark/light theme toggle

## License

MIT

## Author

Built for personal use by Maxym - December 2025

