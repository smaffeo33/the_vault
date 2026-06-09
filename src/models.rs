use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub username: String,
    pub password_plana: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vault {
    pub salt: String,

    pub accounts: HashMap<String, Credential>,
}
impl Vault {
    pub fn new(salt: String) -> Self {
        Self {
            salt,
            accounts: HashMap::new(),
        }
    }
}