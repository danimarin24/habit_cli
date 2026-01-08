use std::io::{self, Write};

enum FrequencyHabit {
    Daily,
    Weekly,
    Monthly,
}

struct Habit {
    name: String,
    frequency: FrequencyHabit,
    count: u32,
}

enum MenuOption {
    Help,
    AddHabit,
    RemoveHabit,
    Exit,
}

fn main() {
    loop {
        match choice_option() {
            Some(MenuOption::Help) => {
                show_help();
            }
            Some(MenuOption::AddHabit) => {
                println!("Adding Habit");
            }
            Some(MenuOption::RemoveHabit) => {
                println!("Removing habit");
            }
            Some(MenuOption::Exit) => {
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
    println!("\texit\t\tExit program.");
}

fn choice_option() -> Option<MenuOption> {
    let input = read_line();
    match input.as_str() {
        "help" => Some(MenuOption::Help),
        "add" => Some(MenuOption::AddHabit),
        "remove" => Some(MenuOption::RemoveHabit),
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
