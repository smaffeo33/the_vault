use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use crate::crypto::save_vault_in_disk;
use crate::models;
use crate::models::Vault;

pub fn print_help() {
    let help_text = r#"Available commands:
    - get <service_name>: Retrieve credentials for a specific service.
    - all: List all stored services.
    - add: Add new credentials for a service.
    - edit <service_name>: Edit credentials for a specific service.
    - delete <service_name>: Delete credentials for a specific service.
    - help: Show this help message.
    - clear: clears the screen.
    - q: Quit the application.
    "#;
    println!("{}", help_text);
}

pub fn delete(arg: Option<&str>, vault: &mut Vault, key: &[u8;32]) {
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

pub fn add(vault: &mut Vault, key: &[u8; 32]) {
    let mut rl = DefaultEditor::new().expect("Failed to initialize readline");

    println!("[!] Service name:");
    let service_name = match read_input(&mut rl) {
        Some(name) => name,
        None => return,
    };

    if vault.accounts.get(&service_name).is_some() {
        println!("[!] Service '{}' already exists. Use 'edit' command to modify it.", service_name);
        return;
    }

    println!("[!] Add new username for {}:", service_name);
    let username = match read_input(&mut rl) {
        Some(user) => user,
        None => return,
    };

    println!("[!] Add new password for {} with username {}:", service_name, username);
    let password = match read_input(&mut rl) {
        Some(pass) => pass,
        None => return,
    };

    vault.accounts.insert(
        service_name,
        models::Credential {
            username,
            password_plana: password,
        },
    );

    save_vault_in_disk(vault, key);
    println!("[✔] Service added successfully.");
}

fn read_input(rl: &mut DefaultEditor) -> Option<String> {
    match rl.readline("> ") {
        Ok(input) => {
            let cleaned = input.trim().to_string();
            if cleaned.is_empty() {
                return None;
            }
            Some(cleaned)
        }
        Err(ReadlineError::Interrupted) => {
            println!("[!] Interrupted (Ctrl+C). Exiting...");
            None
        }
        Err(ReadlineError::Eof) => {
            println!("[!] EOF (Ctrl+D). Exiting...");
            None
        }
        Err(err) => {
            println!("[!] Error reading input: {:?}", err);
            None
        }
    }
}

pub fn edit( vault: &mut Vault, key: &[u8;32] ) {
    let mut rl = DefaultEditor::new().expect("Failed to initialize readline");


    println!("[!] Which service do you want to modify:");


    let service = match read_input(&mut rl) {
        Some(service) => service,
        None => return,
    };

    if !vault.accounts.contains_key(&service) {
        println!("[!] Unknown service '{}' does not exists.", service);
        return;
    }

    println!("[!] What do you want to edit: (service_name, username, password, help or q to quit");


    let command = match read_input(&mut rl) {
        Some(command) => command,
        None => return,
    };

    match command.as_str() {
        "service_name" => {
            println!("[!] Enter the new service name:");

            let new_name = match read_input(&mut rl){
                Some(name) => name,
                None => return,
            };

            if let Some(credential_backup) = vault.accounts.remove(&service) {
                vault.accounts.insert(new_name, credential_backup);
                println!("[✔] Service name updated successfully.");
                save_vault_in_disk(vault, key);
            } else {
                println!("[!] Error: Service '{}' not found.", service);
            }
        },
        "username" => {
            println!("[!] Enter the new username:");

            let new_name = match read_input(&mut rl){
                Some(new_name) => new_name,
                None => return,
            };

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
        "q" => {return}
        _ => {println!("Invalid command. See help"); return;}
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


pub fn get_all( vault: &Vault) {
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

pub fn get(arg: Option<&str>, vault: &mut Vault) {
    let service_name = match arg {
        Some(name) => name,
        None => {
            println!("[!] Error: Please provide a Service name. (Usage: get <service>)");
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