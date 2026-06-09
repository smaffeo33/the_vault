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

        println!("ANDUVO");
        initialized_vault_case();


    } else {

        let vault: Vault = Vault::new(generate_salt());

        println!("Welcome to Iron Vault!");

        loop {
            println!("Create your Master Password: ");
            print!("> ");
            io::stdout().flush().unwrap();

            let master_pwd = rpassword::read_password().expect("Failed to read password");
            println!("Please repeat your Master Password");
            print!("> ");
            io::stdout().flush().unwrap();

            let repeat_pwd = rpassword::read_password().expect("Failed to read password");

            if (master_pwd != repeat_pwd) {
                println!("[!] Master Passwords do not match. Try again.\n");
                continue;
            }


            let key = crypto::derive_key(&master_pwd, &vault.salt).expect("Failed to derive key");
            let json_text = serde_json::to_string(&vault).expect("Failed to serialize");
            let encrypted_bytes = crypto::encrypt_data(&json_text, &key).expect("Failed to encrypt");
            std::fs::write("vault.data", encrypted_bytes).expect("Failed to write to disk");
            // std::fs::write("/tmp/vault_test.data", encrypted_bytes).expect("Failed to write to disk");
            println!("[✔] Vault created and initialized successfully!");
            break
        }

        initialized_vault_case();
    }
}

fn initialized_vault_case() {}


fn print_banner() {
    let banner = r#"
▄▄▄▄▄▄▄▄▄▄▄  ▄         ▄  ▄▄▄▄▄▄▄▄▄▄▄        ▄               ▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄         ▄  ▄         ▄▄▄▄▄▄▄▄▄▄▄
▐░░░░░░░░░░░▌▐░▌       ▐░▌▐░░░░░░░░░░░▌      ▐░▌             ▐░▌▐░░░░░░░░░░░▌▐░▌       ▐░▌▐░▌       ▐░░░░░░░░░░░▌
 ▀▀▀▀█░█▀▀▀▀ ▐░▌       ▐░▌▐░█▀▀▀▀▀▀▀▀▀        ▐░▌           ▐░▌ ▐░█▀▀▀▀▀▀▀█░▌▐░▌       ▐░▌▐░▌        ▀▀▀▀█░█▀▀▀▀
     ▐░▌     ▐░▌       ▐░▌▐░▌                  ▐░▌         ▐░▌  ▐░▌       ▐░▌▐░▌       ▐░▌▐░▌            ▐░▌
     ▐░▌     ▐░█▄▄▄▄▄▄▄█░▌▐░█▄▄▄▄▄▄▄▄▄          ▐░▌       ▐░▌   ▐░█▄▄▄▄▄▄▄█░▌▐░▌       ▐░▌▐░▌            ▐░▌
     ▐░▌     ▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌          ▐░▌     ▐░▌    ▐░░░░░░░░░░░▌▐░▌       ▐░▌▐░▌            ▐░▌
     ▐░▌     ▐░█▀▀▀▀▀▀▀█░▌▐░█▀▀▀▀▀▀▀▀▀            ▐░▌   ▐░▌     ▐░█▀▀▀▀▀▀▀█░▌▐░▌       ▐░▌▐░▌            ▐░▌
     ▐░▌     ▐░▌       ▐░▌▐░▌                      ▐░▌ ▐░▌      ▐░▌       ▐░▌▐░▌       ▐░▌▐░▌            ▐░▌
     ▐░▌     ▐░▌       ▐░▌▐░█▄▄▄▄▄▄▄▄▄              ▐░▐░▌       ▐░▌       ▐░▌▐░█▄▄▄▄▄▄▄█░▌▐░█▄▄▄▄▄▄▄▄▄   ▐░▌
     ▐░▌     ▐░▌       ▐░▌▐░░░░░░░░░░░▌              ▐░▌        ▐░▌       ▐░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌  ▐░▌
      ▀       ▀         ▀  ▀▀▀▀▀▀▀▀▀▀▀                ▀          ▀         ▀  ▀▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀▀    ▀
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