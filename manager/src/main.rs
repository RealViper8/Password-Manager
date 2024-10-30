#![allow(unused)]
use clap::Parser;
use clap::ValueEnum;
use crossterm::style::ContentStyle;
use crossterm::style::Stylize;
use crossterm::Command;
use crossterm::{
    execute,
    style::{Color, PrintStyledContent, StyledContent},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use cryptlib::encode::Encoder;
use cryptlib::fm::FileManager;
use cryptlib::json::Account;
use cryptlib::json::AccountManager;
use cryptlib::json::UserInputAccount;
use std::env;
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "Password Manager")]
#[command(version = "0.0.1")]
#[command(about = "A simple to use, efficient, and full-featured Command Line Password Manager")]
struct Args {
    #[arg(value_enum)]
    action: ActionType,

    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    email: String,

    #[arg(short, long)]
    password: String,

    #[arg(short, long)]
    id: usize,
}

#[derive(ValueEnum, Debug, Clone)]
enum ActionType {
    Add,
    Remove,
    Edit,
}

type PasswordEncoder = Encoder;
const SAVE_FILENAME: &str = "accounts_data.json";

fn main() {
    let args = env::args().collect::<Vec<String>>().len();

    let mut encoder: PasswordEncoder = Encoder::default();
    let mut accounts: AccountManager = AccountManager::default();
    let mut fm: FileManager = FileManager::default();

    if (Path::new(SAVE_FILENAME).exists()) {
        accounts.read(SAVE_FILENAME);
    }

    if (args > 1) {
        let cli = Args::parse();
        match cli.action {
            ActionType::Add => {
                println!("Adding user");
                println!("Name: {}", cli.name);
                println!("Email: {}", cli.email);
                println!("Password: {}", cli.password);

                accounts.add(Account {
                    user_id: None,
                    email: Some(cli.email),
                    username: cli.name,
                    password: cli.password,
                });
            }
            ActionType::Edit => {
                println!("Updating User");
                println!("UUID: {}", cli.id);
                println!("Name: {}", cli.name);
                println!("Email: {}", cli.email);
                println!("Password: {}", cli.password);

                if let Err(e) = accounts.edit(
                    Account {
                        user_id: None,
                        email: Some(cli.email),
                        password: cli.password,
                        username: cli.name,
                    },
                    cli.id,
                ) {
                    eprintln!("[ERROR] {e}");
                }
            }
            ActionType::Remove => {
                println!("Removing User");
                println!("UUID: {}", cli.id);
                println!("Name: {}", cli.name);
                println!("Email: {}", cli.email);
                println!("Password: {}", cli.password);

                accounts.remove(cli.id);
            }
        };
    } else {
        let mut input: String = String::new();
        let mut stdout = io::stdout();
        execute!(stdout, terminal::Clear(ClearType::All));
        println!("\n\x1b[1;32m=====\x1b[0m \x1b[0;36mPassword Manager \x1b[1;32m=====\x1b[0m");
        println!("\t\x1b[0;32mVersion 0.0.1 \x1b[0m");

        loop {
            print!("\x1b[0;36m@ \x1b[1;32m > \x1b[0;36m");
            stdout.flush().unwrap();

            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let trimmed = input.trim();
            if trimmed.is_empty() {
                continue;
            }

            let args: Vec<&str> = trimmed.split_whitespace().collect();

            match args.len() {
                1 => match args[0] {
                    "help" => {
                        println!(
                            "\x1b[0;32mhelp\t\t\x1b[0;36mDisplays help information about PasswordManagers commands."
                        );
                        println!("\x1b[0;32mls\t\t\x1b[0;36mLists the files and subdirectories of a directory on.");
                        println!("\x1b[0;32mclear\t\t\x1b[0;36mClears the screen content.");
                        println!("\x1b[0;32mcd\t\t\x1b[0;36mDisplays or changes the name of the current directory.");
                        println!("\x1b[0;32mexit\t\t\x1b[0;36mTerminates the program.");
                    }
                    "ls" => {
                        fm.search();
                        println!("{}", fm);
                    }
                    "add" => {
                        let mut input: UserInputAccount = UserInputAccount::default();
                        print!("Name: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input.username).unwrap();
                        print!("Email {}: ", "(Empty for none)".dim());
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input.email).unwrap();
                        print!("\x1b[0;36mPassword: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input.password).unwrap();

                        encoder.set_text(input.password.trim());
                        let mut hashed_password = encoder.hash().unwrap();

                        accounts.accounts.push(Account {
                            email: (!input.email.is_empty())
                                .then_some(String::from(input.email.trim())),
                            password: hashed_password,
                            user_id: Some(accounts.accounts.len() as u32),
                            username: input.username.trim().to_string(),
                        });
                    }
                    "list" | "accounts" => {
                        for account in accounts.accounts.iter() {
                            println!("------");
                            println!("ID:\t\t{}", account.user_id.unwrap_or(0));
                            println!("Name:\t\t{}", account.username);
                            println!("Password:\t\t{}", account.password);
                            println!(
                                "Email:\t\t{}",
                                account.email.as_ref().unwrap_or(&"None".to_string())
                            );
                            println!("------");
                        }
                    }
                    _ => (),
                },
                2 => {
                    if (args[0] == "ls" && args[1].contains("help")) {
                        println!("ls - A command to list files and subdirectories");
                    }
                }
                _ => (),
            }

            if trimmed.eq_ignore_ascii_case("exit") {
                break;
            }
        }

        match accounts.close(SAVE_FILENAME) {
            Ok(()) => (),
            Err(e) => eprintln!("\x1b[1;33m[\x1b[0;33mWARNING\x1b[1;33m] \x1b[0;33m{e}\x1b[0m"),
        }
    }
}
