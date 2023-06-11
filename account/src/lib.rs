use rand::RngCore;
use secp256k1::{ecdsa::Signature, PublicKey, Secp256k1, SecretKey};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub private_key: String,
    pub public_key: String,
    pub secret_key: String,
    pub public_key_bytes: Vec<u8>,
    pub store: HashMap<String, String>,
    pub acc_type: AccountType,
    pub tokens: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    User,
    Contract,
    Node,
    Token {
        init_supply: u128,
        burn: bool,
    },
    SmartContract,
    Validator {
        correctly_validated_blocks: u128,
        incorrectly_validated_blocks: u128,
        you_get_the_idea: bool,
    },
}

impl Account {
    pub fn new(account_type: AccountType) -> Self {
        let (priv_key, pub_key, pk_bytes) = Self::generate_keypair();

        Self {
            private_key: priv_key,
            public_key: pub_key,
            public_key_bytes: pk_bytes.to_vec(),
            store: HashMap::new(),
            acc_type: account_type,
            tokens: 0,
        }
    }

    pub fn from(priv_key: String) -> Self {
        let context = Secp256k1::default();

        let secret_key = SecretKey::from_slice(&priv_key.as_bytes()).unwrap();

        let pub_key = PublicKey::from_secret_key(&context, &secret_key);

        Self {
            private_key: priv_key,
            public_key: pub_key.clone().to_string(),
            public_key_bytes: pub_key.clone().serialize().to_vec(),
            store: HashMap::new(),
            acc_type: AccountType::User,
            tokens: 0,
        }
    }

    pub fn generate_adress(&self) -> String {
        format!("etnl:{}", &self.public_key[33..66])
    }

    pub fn sign(&self) {}

    pub fn verify(&self) {}

    pub fn generate_keypair() -> (String, String, String, [u8; 33]) {
        let mut priv_key = [0; 32];
        rand::thread_rng().fill_bytes(&mut priv_key);

        let context = Secp256k1::default();

        let secret_key = SecretKey::from_slice(&priv_key).unwrap();

        let pub_key = PublicKey::from_secret_key(&context, &secret_key);

        (
            hex::encode(priv_key),
            hex::encode(secret_key.secret_bytes()),
            pub_key.clone().to_string(),
            pub_key.clone().serialize(),
        )
    }
}
