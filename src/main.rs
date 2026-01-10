use std::io::{self, Write};

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
    count: u32,
}

enum MenuOption {
    Help,
    AddHabit,
    RemoveHabit,
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
    println!("\tlist\t\tTo list all habits.");
    println!("\tremove\t\tTo remove one habit.");
    println!("\texit\t\tExit program.");
}

fn choice_option() -> Option<MenuOption> {
    let input = read_line();
    match input.as_str() {
        "help" => Some(MenuOption::Help),
        "add" => Some(MenuOption::AddHabit),
        "remove" => Some(MenuOption::RemoveHabit),
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
    name
        .to_lowercase()
        .replace(' ', "-")
}

fn add_habit(habits: &mut Vec<Habit>) {
    println!("Habit name?");
    let name = read_line();

    let id = make_id(&name);

    if habits.iter().any(|h| h.id == id) {
        println!("Habit already exists.");
        return;
    }

    // Por ahora, default
    let frequency = FrequencyHabit::Daily;
    let count: u32 = 0;

    let habit_add = Habit { id, name, frequency, count };
    habits.push(habit_add);
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


fn list_habits(habits: &[Habit]) {
    if habits.is_empty() {
        println!("No habits yet. Use 'add'.");
        return;
    }

    for (i, habit) in habits.iter().enumerate() {
        println!("{}: [{}] - {} - {:?} - count={}", i + 1, habit.id, habit.name, habit.frequency, habit.count);
    }
}
