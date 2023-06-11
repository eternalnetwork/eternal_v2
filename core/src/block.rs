use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub prev: Option<String>,
    pub hash: Option<String>,
    pub nonce: u128,
}

impl Block {
    pub fn new(prev_hash: Option<String>) -> Self {
        Block {
            nonce: 0,
            hash: None,
            prev: prev_hash,
            transactions: Vec::new(),
        }
    }

    pub fn set_nonce(&mut self, nonce: u128) {
        self.nonce = nonce;
        self.update_hash();
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        for transaction in self.transactions.iter() {
            hasher.update(transaction.calculate_hash())
        }

        let block_as_string = format!("{:?}", (&self.prev, &self.nonce));
        hasher.update(&block_as_string);

        return format!("{:X}", hasher.finalize());
    }

    /// Appends a transaction to the queue
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        self.update_hash();
    }

    /// Will return the amount of transactions
    pub fn get_transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Will update the hash field by including all transactions currently inside
    /// the public modifier is only for the demonstration of attacks
    pub fn update_hash(&mut self) {
        self.hash = Some(self.calculate_hash());
    }

    /// Checks if the hash is set and matches the blocks interna
    pub fn verify_own_hash(&self) -> bool {
        if self.hash.is_some() && // Hash set
            self.hash.as_ref().unwrap().eq(&self.calculate_hash())
        {
            // Hash equals calculated hash

            return true;
        }
        false
    }
}
