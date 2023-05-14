use rand::RngCore;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub private_key: String,
    pub public_key: String,
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
        let (priv_key, pub_key) = Self::generate_keypair();

        Self {
            private_key: priv_key,
            public_key: pub_key,
            store: HashMap::new(),
            acc_type: account_type,
            tokens: 0,
        }
    }

    pub fn generate_adress(&self) -> String {
        format!("ETNL{}", &self.public_key[33..66])
    }

    pub fn generate_keypair() -> (String, String) {
        let mut priv_key = [0; 32];
        rand::thread_rng().fill_bytes(&mut priv_key);

        let context = Secp256k1::default();

        let secret_key = SecretKey::from_slice(&priv_key).unwrap();

        let pub_key = PublicKey::from_secret_key(&context, &secret_key);

        (hex::encode(priv_key), pub_key.to_string())
    }
}
