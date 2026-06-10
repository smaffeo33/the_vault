use crate::commands::{add, delete, edit, generate, get, get_all, print_help};
use crate::crypto::{generate_salt, save_vault_in_disk};
use crate::models::Vault;
use crate::utils::{clipboard, print_banner, print_welcome};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io;
use std::io::Write;
use std::path::Path;

mod models;
mod crypto;
mod commands;
mod utils;

const FILE_PATH: &str = "vault.data";

fn main() {
    print_welcome();
    print_banner();


    // let test_FILE_PATH = "/tmp/vault_test.data";

    if Path::new(FILE_PATH).exists() {

        initialized_vault_case();


    } else {

        let mut vault: Vault = Vault::new(generate_salt());

        println!("Welcome to The Iron Vault!");
        let key: [u8; 32];

        loop {
            println!("Create your Master Password: ");
            print!("> ");
            io::stdout().flush().unwrap();

            let master_pwd = rpassword::read_password().expect("Failed to read password");
            println!("Please repeat your Master Password");
            print!("> ");
            io::stdout().flush().unwrap();

            let repeat_pwd = rpassword::read_password().expect("Failed to read password");

            if master_pwd != repeat_pwd {
                println!("[!] Master Passwords do not match. Try again.\n");
                continue;
            }


            key = crypto::derive_key(&master_pwd, &vault.salt).expect("Failed to derive key");

            save_vault_in_disk(&vault, &key);

            println!("[✔] Vault created and initialized successfully!");
            break
        }

        interactive_terminal(&mut vault, &key);
    }
}

fn initialized_vault_case() {
    let file_bytes = std::fs::read(FILE_PATH).unwrap();
    let salt_bytes = &file_bytes[0..32];
    let salt_string = std::str::from_utf8(salt_bytes).unwrap();
    let encrypted_payload = &file_bytes[32..];
    let mut key : [u8;32];

    let mut vault: Vault = loop {
        println!("Please input your Master Password: ");
        print!("> ");
        io::stdout().flush().unwrap();



        let master_pwd = rpassword::read_password().expect("Failed to read password");
        key = crypto::derive_key(&master_pwd, salt_string).expect("Failed to derive key");
        match crypto::decrypt_data(encrypted_payload, &key) {
            Ok(decrypted_payload) => {
                println!("[✔] Access Granted!");

                let v: Vault = serde_json::from_str(&decrypted_payload).expect("Failed to parse Vault data");

                break v;
            }
            Err(_) => {
                println!("[!] Incorrect Master Password. Access Denied. Try again.\n");
            }
        }
    };

    interactive_terminal(&mut vault, &key);
}


fn interactive_terminal(vault: &mut Vault, key: &[u8; 32]) {
    println!("Welcome back to your bunker. Drop your commands below.");

    let mut rl = DefaultEditor::new().expect("Failed to initialize readline");

    let _ = rl.load_history("history.txt");

    loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(input_command) => {
                if input_command.trim().is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(input_command.as_str());

                let mut commands = input_command.split_whitespace();

                match commands.next() {
                    Some("q") => {
                        // if the usage is q <garbage> i dont care, the user wants to exit
                        let _ = rl.save_history("history.txt");
                        return;
                    },
                    Some("clear") => {
                        // if the usage is clear <garbage> i dont care, the user wants to clear

                        print!("\x1B[2J\x1B[1;1H");
                        io::stdout().flush().unwrap();
                        print_banner();
                        continue;
                    },
                    Some("generate") => {
                        let size_arg = commands.next();
                        let length = match size_arg {
                            Some(val) => val.parse::<usize>().unwrap_or(16),
                            None => 16,
                        };
                        let random_pass = generate(length);
                        println!("Your random generated password is {}", random_pass);
                        clipboard(&random_pass);

                    }
                    Some("get") => {
                        let service = commands.next();

                        if commands.next().is_some() {
                            println!("[!] Error: Too many arguments. Usage: get <service_name>");
                            continue;
                        }

                        get(service, vault);
                    },
                    Some("all") => {
                        if commands.next().is_some() {
                            println!("[!] Error: Too many arguments. Usage: all");
                            continue;
                        }

                        get_all(vault)
                    },
                    Some("edit") => {
                        let service = commands.next();


                        if commands.next().is_some() {
                            println!("[!] Error: Too many arguments. Usage: edit <service>");
                            continue;
                        }
                        edit(service, vault, key)
                    },
                    Some("add") => {
                        if commands.next().is_some() {
                            println!("[!] Error: Too many arguments. Usage: all");
                            continue;
                        }
                        add(vault, key)
                    },
                    Some("delete") => {
                        let service = commands.next();

                        if commands.next().is_some() {
                            println!("[!] Error: Too many arguments. Usage: delete <service_name>");
                            continue;
                        }

                        delete(service, vault, key)
                    },
                    Some("help") => print_help(),
                    Some(_) => println!("Unknown command."),
                    _ => {}
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("[!] Interrupted (Ctrl+C). Exiting...");
                let _ = rl.save_history("history.txt");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("[!] EOF (Ctrl+D). Exiting...");
                let _ = rl.save_history("history.txt");
                break;
            },
            Err(err) => {
                println!("[!] Error reading input: {:?}", err);
                break;
            }
        }
    }
}

