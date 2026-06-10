use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use argon2::Argon2;
use crate::FILE_PATH;
use crate::models::Vault;

pub fn generate_salt() -> String {
    let mut salt = [0u8; 16];

    rand::fill(&mut salt);

    hex::encode(&salt)
}

pub fn derive_key(password: &str, salt: &str) -> Result<[u8; 32], String> {

    let mut key = [0u8; 32];

    let argon2 = Argon2::default();
    argon2.hash_password_into(password.as_bytes(), salt.as_bytes(), &mut key)
        .map_err(|e| format!("Error deriving key: {}", e))?;

    Ok(key)
}

pub fn encrypt_data(json_text: &str, key: &[u8]) -> Result<Vec<u8>,  String> {

    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| format!("Error creating cipher: {}", e))?;

    let mut nonce_bytes = [0u8; 12];

    rand::fill(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, json_text.as_bytes()).map_err(|e| format!("Error encrypting data: {}", e))?;

    let mut final_payload = Vec::with_capacity(12 + ciphertext.len());

    final_payload.extend_from_slice(&nonce_bytes);
    final_payload.extend_from_slice(&ciphertext);

    Ok(final_payload)
}

pub fn decrypt_data(encrypted_data: &[u8], key: &[u8; 32]) -> Result<String, String> {

    todo!()
}

pub fn save_vault_in_disk(vault: &Vault, key: &[u8; 32]) {
    let json_text = serde_json::to_string(vault).expect("Failed to serialize");
    let encrypted_bytes = encrypt_data(&json_text, key).expect("Failed to encrypt");

    let mut final_file_bytes = Vec::new();
    final_file_bytes.extend_from_slice(vault.salt.as_bytes());
    final_file_bytes.extend_from_slice(&encrypted_bytes);

    std::fs::write(FILE_PATH, final_file_bytes).expect("Failed to write to disk");
}