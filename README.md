# WorkClock

A simple command-line work session tracker built in C++. Log your work hours by project and date, save them to a file, and view your history.

## Features

- Add work sessions with date, project name, and hours
- View all logged sessions in a formatted table
- Save sessions to file for persistence
- Load sessions from file on startup
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
 4) Save sessions to file
 5) Exit
=======================================
```

## Example Output

```
Date        Projects            Hours   
--------------------------------------
2025-12-06  WorkClock           3       
2025-12-05  CppLearning         2.5     
```

## Concepts Used

- Classes with private members and public getters/setters
- File I/O (`fstream`)
- Vectors
- Input validation
- Formatted output with `iomanip`
- Menu-driven interface

## Author

Built as a C++ learning project — my first functional application!
