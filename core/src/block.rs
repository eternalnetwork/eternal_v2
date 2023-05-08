use crate::{byte_vector_to_string, transaction::Transaction};
use serde::{Deserialize, Serialize};
use blake2::{Blake2s256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// Changes the nonce number and updates the hash
    pub fn set_nonce(&mut self, nonce: u128) {
        self.nonce = nonce;
        self.update_hash();
    }

    /// Will calculate the hash of the whole block including transactions Blake2 hasher
    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2s256::new();

        for transaction in self.transactions.iter() {
            hasher.update(transaction.calculate_hash())
        }

        let block_as_string = format!("{:?}", (&self.prev, &self.nonce));
        hasher.update(&block_as_string);

        let binding = hasher.finalize();
        let hash: &[u8] = binding.as_ref();

        return Vec::from(hash);
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
        self.hash = Some(byte_vector_to_string(&self.calculate_hash()));
    }

    /// Checks if the hash is set and matches the blocks interna
    pub fn verify_own_hash(&self) -> bool {
        if self.hash.is_some() && // Hash set
            self.hash.as_ref().unwrap().eq(
                &byte_vector_to_string(
                    &self.calculate_hash()))
        {
            // Hash equals calculated hash

            return true;
        }
        false
    }
}
