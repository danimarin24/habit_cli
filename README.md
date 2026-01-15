# Habit CLI

A simple command-line application for tracking your daily, weekly, and monthly habits. Built with Rust.

## Features

- Add, remove, and manage habits with different frequencies (daily/weekly/monthly)
- Mark habits as completed for specific dates
- View completion history for any habit
- Track statistics including streaks, total completions, and more
- Automatic persistence to JSON file
- Interactive command-line interface

## Installation

Make sure you have Rust installed, then clone and build the project:

```bash
git clone <repository-url>
cd habit_cli
cargo build --release
```

## Usage

Run the application:

```bash
cargo run
```

### Available Commands

- `help` - Show available commands
- `add` - Create a new habit (daily/weekly/monthly)
- `remove` - Remove a habit by ID
- `check` - Mark a habit as completed for a date
- `history` - View completion history for a habit
- `stats` - Show statistics for a habit (completions, streaks, etc.)
- `list` - List all your habits
- `exit` - Exit the application

### Example Session

```
> add
Habit name? Exercise
Frequency? (daily/weekly/monthly) daily
OK: Added [exercise] Exercise (daily)

> check
Habit id? exercise
Date (YYYY-MM-DD)? (empty=today)
OK: Checked [exercise] Exercise (2026-01-15)

> stats
Habit id? exercise
Stats for [exercise] Exercise:
 - frequency: daily
 - total completions: 1
 - first completion: 2026-01-15
 - last completion: 2026-01-15
 - current streak: 1 day(s)

> list
1: [exercise] - Exercise - daily - completions=1
```

## Data Storage

Habit data is automatically saved to `.habits.json` in the project directory. This file is git-ignored by default to keep your personal data private.

## Technical Details

- **Language**: Rust (edition 2024)
- **Dependencies**:
  - `chrono` - Date and time handling
  - `serde` / `serde_json` - JSON serialization
- **Data Format**: Pretty-printed JSON for easy inspection and backup