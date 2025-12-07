# WorkClock

A command-line work session tracker built in C++. Log your work hours by project and date, track totals, and persist data to a file.

## Features

- Add work sessions with date, project name, and hours
- View all logged sessions in a formatted table
- Get total hours by project name
- Get total hours by date
- Get total hours across all sessions
- Save/load sessions to file for persistence
- Input validation for hours (0-24)

## How to Compile

```bash
g++ -o WorkClock WorkClock.cpp
```

## How to Run

```bash
./WorkClock
```

## Usage

```
============ WORK LOG MENU ============
 1) Add a new session
 2) Show all sessions
 3) Load sessions from file
 4) Get hours by project
 5) Get hours by date
 6) Get total hours
 7) Save sessions to file
 0) Exit
=======================================
```

## Example Output

```
Date        Projects            Hours   
--------------------------------------
2025-12-06  WorkClock           3       
2025-12-05  CppLearning         2.5     

Total hours for WorkClock is: 3
Total hours for all sessions is: 5.5
```

## Concepts Used

- Classes with private members and public getters/setters
- Hash maps (`unordered_map`) for O(1) lookups
- File I/O (`fstream`)
- Vectors
- Input validation
- Formatted output with `iomanip`
- Menu-driven interface

## Author

Built as a C++ learning project — my first functional application!
