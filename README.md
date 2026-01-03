# Chrono

A work session tracker for freelancers and side-hustlers. Track your time, see your earnings, reach your financial goals.

## Features

**Core (Free)**
- Dashboard with daily earnings and hours worked
- Session tracking with custom hourly rates per project type
- Financial goals with progress tracking
- Analytics and earnings visualization
- Achievement system
- All data stored locally - no account required

**Pro**
- Invoice generation from sessions
- AI financial advisor
- Voice input for logging sessions
- Earnings simulator
- Full analytics history

## Download

Grab the latest release for your platform:

- **Windows**: `Chrono_x64-setup.exe` or `Chrono_x64_en-US.msi`
- **macOS (Apple Silicon)**: `Chrono_aarch64.dmg`
- **macOS (Intel)**: `Chrono_x64.dmg`
- **Linux**: `chrono_amd64.deb` or `chrono_amd64.AppImage`

[All releases](https://github.com/skyvaultex/Chrono/releases/latest)

## Tech Stack

- Svelte + TypeScript + Tailwind CSS (frontend)
- Rust + Tauri (backend)
- SQLite (local database)
- Next.js + PostgreSQL (license server)

## Development

```bash
git clone https://github.com/skyvaultex/Chrono.git
cd Chrono/Launcher

npm install
npm run tauri dev

# Build
npm run tauri build
```

## License

MIT
