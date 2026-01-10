use std::io::{self, Write};
use chrono::{NaiveDate, Local};

#[derive(Debug, Clone, Copy)]
enum FrequencyHabit {
    Daily,
    Weekly,
    Monthly,
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
    ListHabits,
    Exit,
}

fn main() {
    let mut habits: Vec<Habit> = Vec::new();

    loop {
        match choice_option() {
            Some(MenuOption::Help) => {
                show_help();
            }
            Some(MenuOption::AddHabit) => {
                println!("Adding Habit");
                add_habit(&mut habits);
            }
            Some(MenuOption::RemoveHabit) => {
                println!("Removing habit");
                remove_habit(&mut habits);
            }
            Some(MenuOption::CheckHabit) => {
                println!("Checking habit");
                check_habit(&mut habits);
            }
            Some(MenuOption::ListHabits) => {
                println!("Listing habits");
                list_habits(&habits);
            }
            Some(MenuOption::Exit) => {
                println!("Exiting...");
                break;
            }
            None => {
                println!("Invalid option. Type 'help'.");
            }
        }
    }
}

fn show_help() {
    println!("USAGE");
    println!("\t$ habit [COMMAND]");
    println!("COMMANDS");
    println!("\thelp\t\tShow this help.");
    println!("\tadd\t\tTo create new habit.");
    println!("\tremove\t\tTo remove one habit.");
    println!("\tcheck\t\tTo check habit.");
    println!("\tlist\t\tTo list all habits.");
    println!("\texit\t\tExit program.");
}

fn choice_option() -> Option<MenuOption> {
    let input = read_line();
    match input.as_str() {
        "help" => Some(MenuOption::Help),
        "add" => Some(MenuOption::AddHabit),
        "remove" => Some(MenuOption::RemoveHabit),
        "check" => Some(MenuOption::CheckHabit),
        "list" => Some(MenuOption::ListHabits),
        "exit" | "quit" => Some(MenuOption::Exit),
        _ => None,
    }
}

fn read_line() -> String {
    let mut input = String::new();

    print!("> ");
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

// Habit
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
    println!("Habit name?");
    let name = read_line();

    if name.is_empty() {
        println!("Habit name cannot be empty.");
        return;
    }    

    let id = make_id(&name);

    if id.is_empty() {
        println!("Habit name produced an empty id. Use letters/numbers.");
        return;
    }
    

    if habits.iter().any(|h| h.id == id) {
        println!("Habit already exists.");
        return;
    }

    // Por ahora, default
    let frequency = FrequencyHabit::Daily;
    let completions: Vec<NaiveDate> = Vec::new();

    habits.push(Habit { id, name, frequency, completions });
}

fn remove_habit(habits: &mut Vec<Habit>) {
    println!("Habit id to remove?");
    let id = read_line();

    // Find index
    if let Some(index) = habits.iter().position(|h| h.id == id) {
        let removed = habits.remove(index);
        println!("Removed habit: [{}] {}", removed.id, removed.name);
    } else {
        println!("Habit not found: {}", id);
    }
}

fn check_habit(habits: &mut [Habit]) {
    println!("Habit id?");
    let id = read_line();

    let Some(habit) = habits.iter_mut().find(|h| h.id == id) else {
        println!("Habit not found: {}", id);
        return;
    };

    println!("Date (YYYY-MM-DD)? (empty = today)");
    let date_str = read_line();
    
    let date = if date_str.is_empty() {
        Local::now().date_naive()
    } else {
        match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => {
                println!("Invalid date. Example: 2026-01-10");
                return;
            }
        }
    };

    if habit.completions.contains(&date) {
        println!("Already checked for {}", date);
        return;
    }
    
    habit.completions.push(date);
    println!("Checked [{}] {}.", habit.id, habit.name);
}



fn list_habits(habits: &[Habit]) {
    if habits.is_empty() {
        println!("No habits yet. Use 'add'.");
        return;
    }

    for (i, habit) in habits.iter().enumerate() {
        println!(
            "{}: [{}] - {} - {:?} - completions={}",
            i + 1,
            habit.id,
            habit.name,
            habit.frequency,
            habit.completions.len()
        );
    }
}
