use std::collections::HashMap;
pub mod wallet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    // pub address: String,

    pub store: HashMap<String, String>,

    pub acc_type: AccountType,

    pub tokens: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    User,
    Contract,
    Token {
        init_supply: u128,
        burn: bool,
    },
    Validator {
        correctly_validated_blocks: u128,
        incorrectly_validated_blocks: u128,
        you_get_the_idea: bool,
    },
}

impl Account {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            // address: String::from_utf8(hash.to_vec()).unwrap(),
            store: HashMap::new(),
            acc_type: account_type,
            tokens: 0,
        }
    }
}
