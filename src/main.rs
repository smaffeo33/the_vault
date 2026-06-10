use std::path::Path;
use crate::models::Vault;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use crate::crypto::generate_salt;

mod models;
mod crypto;

fn main() {
    print_welcome();
    print_banner();


    let file_path = "vault.data";
    // let test_file_path = "/tmp/vault_test.data";

    if Path::new(file_path).exists() {
        // [CASO A] El archivo existe.
        // 1. Pedir Master Password.
        // 2. Leer bytes de vault.data.
        // 3. Desencriptar JSON -> Parsear a la estructura Vault.

        initialized_vault_case(&file_path);


    } else {

        let vault: Vault = Vault::new(generate_salt());

        println!("Welcome to The Iron Vault!");

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


            let key = crypto::derive_key(&master_pwd, &vault.salt).expect("Failed to derive key");
            let json_text = serde_json::to_string(&vault).expect("Failed to serialize");
            let encrypted_bytes = crypto::encrypt_data(&json_text, &key).expect("Failed to encrypt");

            let mut final_file_bytes = Vec::new();
            final_file_bytes.extend_from_slice(vault.salt.as_bytes());
            final_file_bytes.extend_from_slice(&encrypted_bytes);

            std::fs::write(file_path, final_file_bytes).expect("Failed to write to disk");
            // std::fs::write("/tmp/vault_test.data", encrypted_bytes).expect("Failed to write to disk");
            println!("[✔] Vault created and initialized successfully!");
            break
        }

        interactive_terminal(&vault);
    }
}

fn initialized_vault_case(file_path: &str) {
    let file_bytes = std::fs::read(file_path).unwrap();
    let salt_bytes = &file_bytes[0..32];
    let salt_string = std::str::from_utf8(salt_bytes).unwrap();
    let encrypted_payload = &file_bytes[32..];

    let vault : Vault = loop {
        println!("Please input your Master Password: ");
        print!("> ");
        io::stdout().flush().unwrap();



        let master_pwd = rpassword::read_password().expect("Failed to read password");
        let key = crypto::derive_key(&master_pwd, salt_string).expect("Failed to derive key");
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

    interactive_terminal(&vault);
}


fn interactive_terminal(vault: &Vault) {
    println!("Welcome back to your bunker. Drop your commands below.");

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