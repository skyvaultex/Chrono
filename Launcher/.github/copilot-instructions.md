# Chrono - Tauri + Rust + Svelte Project

## Project Overview
Chrono is a desktop work session tracker and financial goal manager built with:
- **Backend**: Tauri + Rust
- **Frontend**: Svelte + TypeScript + Tailwind CSS + Skeleton UI
- **Database**: SQLite via rusqlite

## Tech Stack Rationale
- **Svelte over React**: Simpler syntax, less boilerplate, perfect for forms and tables, smaller bundle size
- **Skeleton UI**: Tailwind-based component library designed specifically for Svelte
- **rusqlite**: Simpler than sqlx for this use case, synchronous API suitable for local desktop app

## Core Features
1. **Work Session Tracking**
   - Construction sessions (auto-calculate pay at $30/hour)
   - Coding sessions (with language and description)
   - Smart project selector with autocomplete

2. **Financial Goals**
   - Track debts, purchases, and savings
   - Progress bars and ETA calculations
   - Contribution tracking

3. **Dashboard**
   - Today's hours and pay
   - Recent sessions
   - Quick actions

4. **Analytics** ✅
   - Summary cards (total time, sessions, avg/longest session, total pay)
   - Range selector (Today, 7D, 30D, 90D, Year, All)
   - Line chart: hours trend over time (ApexCharts)
   - Donut chart: time distribution by category
   - Bar chart: hours by weekday
   - Category breakdown table

5. **AI Advisor**
   - Summary display
   - Question input area
   - Response panel (ChatGPT integration)

## Setup Progress
- ✅ Created copilot-instructions.md
- ✅ Project structure setup
- ✅ Analytics tab with ApexCharts
