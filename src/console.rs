use duckdb::Error;
use std::io::Write;

use crate::database::connection::{Database, User};

const EXIT_CMD: &str = "exit";
const HELP_CMD: &str = "help";
const REGISTER_CMD: &str = "register";
const ACCOUNTS_CMD: &str = "accounts";

// Functions to run the main console.
pub fn run_console(db: &Database) {
    let mut exit: bool = false;
    while !exit {
        let mut command = String::new();
        print!("Enter a command (help for full list): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut command)
            .expect("Console error.");
        let trimmed_input: &str = command.trim();

        match trimmed_input {
            EXIT_CMD => exit = true,
            HELP_CMD => print_help(),
            REGISTER_CMD => register(db),
            _ => eprintln!("Unknown command: '{}'", trimmed_input),
        }
    }
}

fn print_help() {
    println!(
        "Available commands:\n\texit - closes the bank terminal\n\tregister - creates a new user"
    );
}

// Registers a new user. If the provided userID exists, nothing happens and an error is logged.
fn register(db: &Database) {
    let mut name: String = String::new();
    print!("Enter your name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Console error.");
    name = name.trim().to_string();

    let mut user_id: String = String::new();
    print!("Enter your username (no spaces/special chars): ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut user_id)
        .expect("Console error.");
    user_id = user_id.trim().to_string();

    let user: User = User {
        user_id,
        name: name.clone(),
        accounts: Vec::new(),
    };

    let result: Result<usize, Error> = db.add_user(user);
    match result {
        Ok(num_rows) => {
            if num_rows != 1 {
                eprintln!("This userID already exists. Please choose a new one.");
                return;
            }

            println!("Welcome {}, you are now registered.", name)
        }
        Err(error) => eprintln!("Failed to register new user: {}", error),
    };
}
