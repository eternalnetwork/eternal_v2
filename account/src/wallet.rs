use bip32::Mnemonic;
use crypto::ed25519::keypair;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

use crate::Account;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub mnemonic: String,
    pub private_key: String,
    pub public_key: String,

    pub accounts: Vec<Account>,
}

impl Wallet {
    pub fn new() -> Self {
        let mnemonic = Mnemonic::random(&mut OsRng, Default::default());
        let keypairs = keypair(mnemonic.to_seed("password").as_bytes());
       
        let binding = keypairs.0.to_vec();
        let priv_key = match std::str::from_utf8(&binding) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let pub_key = match std::str::from_utf8(&keypairs.1) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        Self {
            private_key: String::from(priv_key),
            public_key: String::from(pub_key),
            mnemonic: mnemonic.phrase().to_string(),
            accounts: vec![],
        }
    }
}
