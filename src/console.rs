use duckdb::Error;
use std::io::Write;
use std::{fmt::Write as FmtWrite, num::ParseIntError};

use crate::database::connection::{Account, Database, User};

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
            ACCOUNTS_CMD => accounts(db),
            _ => eprintln!(
                "Unknown command: '{}'. Try 'help' for a full list.",
                trimmed_input
            ),
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
    // Get required registration information:
    let mut name: String = String::new();
    print!("Enter your name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Console error.");
    name = name.trim().to_string();

    let user_id: String = get_user_id();

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

fn accounts(db: &Database) {
    // Fetch available accounts and list them:
    let user_id: String = get_user_id();
    let user: Result<User, Error> = db.get_user(user_id);
    match user {
        Err(error) => {
            eprintln!("Failed to fetch a user: {}", error);
            return;
        }
        Ok(user) => account_details(db, user),
    }
}

fn account_details(db: &Database, user: User) {
    println!(
        "\nWelcome {}, you have {} accounts. Select an option:",
        user.name,
        user.accounts.len()
    );
    let mut options_msg: String = String::new();
    let mut num_options: u8 = 0;
    for account_id in &user.accounts {
        writeln!(
            &mut options_msg,
            "\t{} - Account #{}",
            num_options, account_id
        )
        .unwrap();
        num_options += 1;
    }
    writeln!(&mut options_msg, "\t{} - Create new account", num_options).unwrap();
    println!("{}", options_msg);

    // Get the selected option
    let mut selected_option_str: String = String::new();
    print!("Select an option [0-{}]: ", num_options);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut selected_option_str)
        .expect("Console error.");
    selected_option_str = selected_option_str.trim().to_string();
    // Validate the selected option
    let selected_option_res: Result<u8, ParseIntError> = selected_option_str.parse();
    if selected_option_res.is_err() {
        eprintln!("Not a valid number.");
        return;
    }
    let selected_option: u8 = selected_option_res.unwrap();
    if usize::from(selected_option) > user.accounts.len() {
        eprintln!("Invalid selection.");
        return;
    }

    // We have a valid selection. Check whether we are creating or retrieving an account.
    if selected_option == num_options {
        // Create an account

    }
}

// Helper function gets the user ID input from the console.
fn get_user_id() -> String {
    let mut user_id: String = String::new();
    print!("Enter your username (no spaces/special chars): ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut user_id)
        .expect("Console error.");
    user_id.trim().to_string()
}

// TODO: Finish this
// fn create_account() -> Account {

// }
