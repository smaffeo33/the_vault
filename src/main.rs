use crate::crypto::{generate_salt, save_vault_in_disk};
use crate::models::Vault;
use std::io;
use std::io::Write;
use std::path::Path;

mod models;
mod crypto;

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

            // std::fs::write("/tmp/vault_test.data", encrypted_bytes).expect("Failed to write to disk");
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

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input_command = String::new(); io::stdin().read_line(&mut input_command).unwrap();

        let mut commands = input_command.split_whitespace();

        //TODO: clean the iter of commands (Do not let them write more arguments)

        match commands.next() {
            Some("q")  => return,
            Some("get") => get(commands.next(), vault),
            Some("all") => get_all(vault),
            Some("edit") => edit(vault, key),
            Some("add") => add(vault, key),
            Some("delete") => delete(commands.next(), vault, key),
            Some("help") => print_help(),
            Some(_) => {println!("Unknown command."); continue}
            None => {println!("Command is needed"); continue}
        }
    }

}

fn print_help() {
    let help_text = r#"Available commands:
    - get <service_name>: Retrieve credentials for a specific service.
    - all: List all stored services.
    - add: Add new credentials for a service.
    - edit <service_name>: Edit credentials for a specific service.
    - delete <service_name>: Delete credentials for a specific service.
    - help: Show this help message.
    - q: Quit the application.
    "#;
    println!("{}", help_text);
}

fn delete(arg: Option<&str>, vault: &mut Vault, key: &[u8;32]) {
    let service_name = match arg {
        Some(name) => name,
        None => {
            println!("[!] Error: Service name to be deleted is required. (Usage: delete <service>)");
            return;
        }
    };

    if vault.accounts.remove(service_name).is_some() {
        save_vault_in_disk(vault, key);
        //if it gets here then it was deleted succeffully.
        println!("[✔] Service '{}' deleted successfully.", service_name);

    } else {
        println!("[!] Service '{}' not found.", service_name);
    }
}

fn add( vault: &mut Vault, key: &[u8;32] ) {

    println!("[!] Add new credentials for a service:");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut service = String::new();
    io::stdin().read_line(&mut service).unwrap();
    let service = service.trim().to_string();

    if vault.accounts.get(&service).is_some() {
        println!("[!] Service '{}' already exists. Use 'edit' command to modify it.", service);
        return;
    }

    println!("[!] Add new username for {}:", service);
    print!("> ");
    io::stdout().flush().unwrap();

    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    println!("[!] Add new password for {} with username {}:", service, username);
    print!("> ");
    io::stdout().flush().unwrap();

    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();


    vault.accounts.insert(service.trim().to_string(), models::Credential {
        username: username.trim().to_string(),
        password_plana: password.trim().to_string(),
    });
    save_vault_in_disk(vault, key);
     println!("[✔] Service added successfully.");


}

fn edit( vault: &mut Vault, key: &[u8;32] ) {
    println!("[!] Which service do you want to modify:");
    print!("> ");
    io::stdout().flush().unwrap();


    let mut service_input  = String::new();
    io::stdin().read_line(&mut service_input).unwrap();
    let service = service_input.trim().to_string();

    println!("[!] What do you want to edit:");
    print!("> ");

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    match command.trim() {
        "service_name" => {
            println!("[!] Enter the new service name:");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut new_name = String::new();
            io::stdin().read_line(&mut new_name).unwrap();

            if let Some(credential_backup) = vault.accounts.remove(&service) {
                vault.accounts.insert(new_name.trim().to_string(), credential_backup);
                println!("[✔] Service name updated successfully.");

                save_vault_in_disk(vault, key);
            } else {
                println!("[!] Error: Service '{}' not found.", service);
            }
        },
        "username" => {
            println!("[!] Enter the new username:");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut new_name = String::new();
            io::stdin().read_line(&mut new_name).unwrap();
            let new_name = new_name.trim().to_string();

            if let Some(credential_backup) = vault.accounts.remove(&service) {
                vault.accounts.insert(
                    service.clone(),
                    models::Credential {
                        username: new_name,
                        password_plana: credential_backup.password_plana,
                    }
                );
                println!("[✔] Username updated successfully.");

                save_vault_in_disk(vault, key);
            } else {
                println!("[!] Error: Username could not be updated.");
            }
        },
        "password" => {
            println!("[!] Enter the new password:");
            print!("> ");
            io::stdout().flush().unwrap();

            let new_pass = rpassword::read_password().expect("Failed to read password");

            if let Some(credential_backup) = vault.accounts.remove(&service) {
                vault.accounts.insert(service.trim().to_string(), models::Credential{
                    username: credential_backup.username,
                    password_plana: new_pass.trim().to_string(),
                });
                println!("[✔] Password updated successfully.");

                save_vault_in_disk(vault, key);
            } else {
                println!("[!] Error: Password could not be updated.");
            }
        },
        "help" => {print_edit_help()},
        "quit" => {return}
        _ => {println!("Invalid command. See edit help"); return;}
    }

}

fn print_edit_help() {
    let help_text = r#"Available commands:
    - service_name: Edit the name of the service.
    - username: Edit the username for the service.
    - password: Edit the password for the password.
    - help: Show this help message.
    - quit: Quit the edit interface.
    "#;
    println!("{}", help_text);
}


fn get_all( vault: &Vault) {
    if vault.accounts.is_empty() {
        println!("[!] No services stored in the vault.");
        return;
    }


    for (service, credentials) in &vault.accounts {
        println!("Service: {}", service);
        println!("username: {} - password: {}", credentials.username, credentials.password_plana.replace(&credentials.password_plana, "********"));
        println!("==============================");

    }
}

fn get(arg: Option<&str>, vault: &mut Vault) {
    let service_name = match arg {
        Some(name) => name,
        None => {
            println!("[!] Error: Service name to be gotten is required. (Usage: get <service>)");
            return;
        }
    };

    if let Some(credential) = vault.accounts.get(service_name) {
        println!("[✔] Service '{}' found.", service_name);
        println!("Username: {}", credential.username);
        println!("Password: {}", credential.password_plana);
    } else {
        println!("[!] Service '{}' not found.", service_name);

    }

}

fn print_banner() {
    let banner = r#"░██████████░██                      ░██                                                                   ░██    ░██
    ░██    ░██                                                                                            ░██    ░██
    ░██    ░████████   ░███████     ░██░██░████  ░███████  ░████████     ░██    ░██  ░██████   ░██    ░██ ░██ ░████████
    ░██    ░██    ░██ ░██    ░██    ░██░███     ░██    ░██ ░██    ░██    ░██    ░██       ░██  ░██    ░██ ░██    ░██
    ░██    ░██    ░██ ░█████████    ░██░██      ░██    ░██ ░██    ░██     ░██  ░██   ░███████  ░██    ░██ ░██    ░██
    ░██    ░██    ░██ ░██           ░██░██      ░██    ░██ ░██    ░██      ░██░██   ░██   ░██  ░██   ░███ ░██    ░██
    ░██    ░██    ░██  ░███████     ░██░██       ░███████  ░██    ░██       ░███     ░█████░██  ░█████░██ ░██     ░████



               ░████
              ░██
 ░███████  ░████████
░██    ░██    ░██
░██    ░██    ░██
░██    ░██    ░██
 ░███████     ░██



░██
░██
░████████  ░██░████  ░██████    ░██████   ░██    ░██  ░███████   ░███████
░██    ░██ ░███           ░██        ░██  ░██    ░██ ░██    ░██ ░██
░██    ░██ ░██       ░███████   ░███████   ░██  ░██  ░██    ░██  ░███████
░███   ░██ ░██      ░██   ░██  ░██   ░██    ░██░██   ░██    ░██        ░██
░██░█████  ░██       ░█████░██  ░█████░██    ░███     ░███████   ░███████


                                                                                                                        "#;
    println!("{}", banner);
}

fn print_welcome() {
    let banner = r#"▖  ▖  ▜            ▗
▌▞▖▌█▌▐ ▛▘▛▌▛▛▌█▌  ▜▘▛▌▖
▛ ▝▌▙▖▐▖▙▖▙▌▌▌▌▙▖  ▐▖▙▌▖
                        "#;

    println!("{}", banner);
}