use std::io::{self, Write};
use chrono::{NaiveDate, Local};
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum FrequencyHabit {
    Daily,
    Weekly,
    Monthly,
}

impl FrequencyHabit {
    fn from_str(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "daily" => Some(Self::Daily),
            "weekly" => Some(Self::Weekly),
            "monthly" => Some(Self::Monthly),
            _ => None,
        }
    }
}

impl fmt::Display for FrequencyHabit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
        };
        write!(f, "{}", text)
    }
}

#[derive(Debug)]
struct Habit {
    id: String,
    name: String,
    frequency: FrequencyHabit,
    completions: Vec<NaiveDate>,
}

enum MenuOption {
    Help,
    AddHabit,
    RemoveHabit,
    CheckHabit,
    HistoryHabit,
    ListHabits,
    Exit,
}

fn main() {
    let mut habits: Vec<Habit> = Vec::new();

    loop {
        match choice_option() {
            Some(MenuOption::Help) => show_help(),
            Some(MenuOption::AddHabit) => add_habit(&mut habits),
            Some(MenuOption::RemoveHabit) => remove_habit(&mut habits),
            Some(MenuOption::CheckHabit) => check_habit(&mut habits),
            Some(MenuOption::HistoryHabit) => show_history(&habits),
            Some(MenuOption::ListHabits) => list_habits(&habits),
            Some(MenuOption::Exit) => {
                println!("Bye.");
                break;
            }
            None => println!("ERR: Invalid option. Type 'help'."),
        }
    }
}

fn show_help() {
    println!("USAGE");
    println!("\t$ habit [COMMAND]");
    println!("COMMANDS");
    println!("\thelp\t\tShow this help.");
    println!("\tadd\t\tCreate a new habit.");
    println!("\tremove\t\tRemove one habit by id.");
    println!("\tcheck\t\tMark a habit as completed for a date.");
    println!("\thistory\t\tShow habit completion history.");
    println!("\tlist\t\tList all habits.");
    println!("\texit\t\tExit program.");
}

fn choice_option() -> Option<MenuOption> {
    let input = cmd_prompt();
    match input.as_str() {
        "help" => Some(MenuOption::Help),
        "add" => Some(MenuOption::AddHabit),
        "remove" => Some(MenuOption::RemoveHabit),
        "check" => Some(MenuOption::CheckHabit),
        "history" => Some(MenuOption::HistoryHabit),
        "list" => Some(MenuOption::ListHabits),
        "exit" | "quit" => Some(MenuOption::Exit),
        _ => None,
    }
}

fn read_line(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn cmd_prompt() -> String {
    read_line("> ")
}

fn ask(prompt: &str) -> String {
    read_line(&format!("{} ", prompt))
}

fn make_id(name: &str) -> String {
    let mut result = String::new();
    let mut last_was_dash = false;

    for c in name.to_lowercase().chars() {
        let normalized = match c {
            'á' | 'à' | 'ä' | 'â' => 'a',
            'é' | 'è' | 'ë' | 'ê' => 'e',
            'í' | 'ì' | 'ï' | 'î' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' => 'o',
            'ú' | 'ù' | 'ü' | 'û' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            _ => c,
        };

        if normalized.is_ascii_alphanumeric() {
            result.push(normalized);
            last_was_dash = false;
        } else if !last_was_dash {
            result.push('-');
            last_was_dash = true;
        }
    }

    result.trim_matches('-').to_string()
}

fn add_habit(habits: &mut Vec<Habit>) {
    let name = ask("Habit name?");

    if name.is_empty() {
        println!("ERR: Habit name cannot be empty.");
        return;
    }

    let id = make_id(&name);

    if id.is_empty() {
        println!("ERR: Habit name produced an empty id. Use letters/numbers.");
        return;
    }

    if habits.iter().any(|h| h.id == id) {
        println!("ERR: Habit already exists (id: {}).", id);
        return;
    }

    // Keep it simple: ask once, fail if invalid (you can turn this into a retry loop later)
    let freq_input = ask("Frequency? (daily/weekly/monthly)");
    let frequency = match FrequencyHabit::from_str(&freq_input) {
        Some(f) => f,
        None => {
            println!("ERR: Invalid frequency. Use: daily, weekly or monthly.");
            return;
        }
    };

    let completions: Vec<NaiveDate> = Vec::new();
    habits.push(Habit {
        id: id.clone(),
        name: name.clone(),
        frequency,
        completions,
    });

    println!("OK: Added [{}] {} ({})", id, name, frequency);
}

fn remove_habit(habits: &mut Vec<Habit>) {
    let id = ask("Habit id to remove?");

    if let Some(index) = habits.iter().position(|h| h.id == id) {
        let removed = habits.remove(index);
        println!("OK: Removed [{}] {}", removed.id, removed.name);
    } else {
        println!("ERR: Habit not found: {}", id);
    }
}

fn check_habit(habits: &mut [Habit]) {
    let id = ask("Habit id?");

    let Some(habit) = habits.iter_mut().find(|h| h.id == id) else {
        println!("ERR: Habit not found: {}", id);
        return;
    };

    let date_str = ask("Date (YYYY-MM-DD)? (empty=today)");

    let date = if date_str.is_empty() {
        Local::now().date_naive()
    } else {
        match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => {
                println!("ERR: Invalid date. Example: 2026-01-10");
                return;
            }
        }
    };

    if habit.completions.contains(&date) {
        println!("ERR: Already checked for {}", date.format("%Y-%m-%d"));
        return;
    }

    habit.completions.push(date);
    habit.completions.sort();
    habit.completions.dedup();

    println!("OK: Checked [{}] {} ({})", habit.id, habit.name, date.format("%Y-%m-%d"));
}

fn show_history(habits: &[Habit]) {
    let id = ask("Habit id?");

    let Some(habit) = habits.iter().find(|h| h.id == id) else {
        println!("ERR: Habit not found: {}", id);
        return;
    };

    if habit.completions.is_empty() {
        println!("OK: No completions yet for [{}] {}.", habit.id, habit.name);
        return;
    }

    let mut dates = habit.completions.clone();
    dates.sort();

    println!("History for [{}] {}:", habit.id, habit.name);
    for d in dates {
        println!(" - {}", d.format("%Y-%m-%d"));
    }
}

fn list_habits(habits: &[Habit]) {
    if habits.is_empty() {
        println!("OK: No habits yet. Use 'add'.");
        return;
    }

    for (i, habit) in habits.iter().enumerate() {
        println!(
            "{}: [{}] - {} - {} - completions={}",
            i + 1,
            habit.id,
            habit.name,
            habit.frequency,
            habit.completions.len()
        );
    }
}
