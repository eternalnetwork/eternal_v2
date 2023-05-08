use eternal_account::{Account, AccountType};
use serde::{Serialize, Deserialize};

use crate::{block::Block, transaction::Transaction, WorldState};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub accounts: HashMap<String, Account>,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            accounts: HashMap::new(),
            pending_transactions: Vec::new(),
        }
    }

    pub fn append_block(&mut self, block: Block) -> Result<(), String> {

        // The genesis block may create user out of nowhere,
        // and also may do some other things
        let is_genesis = self.len() == 0;

        // Check if the hash matches the transactions
        if !block.verify_own_hash() {
            return Err("The block hash is mismatching! (Code: 93820394)".into());
        }

        // Check if the newly added block is meant to be appended onto the last block
        if !(block.prev == self.get_last_block_hash()) {
            return Err("The new block has to point to the previous block (Code: 3948230)".into());
        }

        // There has to be at least one transaction inside the queue
        if block.get_transaction_count() == 0 {
            return Err("There has to be at least one transaction \
            inside the block! (Code: 9482930)".into());
        }

        // Reject block having nonces that are already used (Prevent reply attacks etc.)
        // @Todo (Will skip that for simplicity)


        // This is expensive and just used for rollback if some transactions succeed whilst
        // others don't (prevent inconsistent states)
        // Arguably, that could be implemented more resource-aware
        let old_state = self.accounts.clone();

        // Execute each transaction
        for (i, transaction) in block.transactions.iter().enumerate() {

            // Execute the transaction
            if let Err(err) = transaction.execute(self, &is_genesis) {
                // Recover state on failure
                self.accounts = old_state;

                // ... and reject the block
                return Err(format!("Could not execute transaction {} due to `{}`. Rolling back \
                (Code: 38203984)", i + 1, err));
            }
        }

        // Everything went fine... append the block
        self.blocks.push(block);

        Ok(())
    }

    /// Will return the amount of blocks currently stored
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Will return the hash of the last block
    pub fn get_last_block_hash(&self) -> Option<String> {
        if self.len() == 0 {
            return None;
        }

        self.blocks[self.len() - 1].hash.clone()
    }

    pub fn check_validity(&self) -> Result<(), String> {
        for (block_num, block) in self.blocks.iter().enumerate() {

            // Check if block saved hash matches to calculated hash
            if !block.verify_own_hash() {
                return Err(format!("Stored hash for Block #{} \
                    does not match calculated hash (Code: 665234234)", block_num + 1).into());
            }

            // Check previous black hash points to actual previous block
            if block_num == 0 {
                // Genesis block should point to nowhere
                if block.prev.is_some() {
                    return Err("The genesis block has a previous hash set which \
                     it shouldn't Code :394823098".into());
                }
            } else {
                // Non genesis blocks should point to previous blocks hash (which is validated before)
                if block.prev.is_none() {
                    return Err(format!("Block #{} has no previous hash set", block_num + 1).into());
                }

                // Store the values locally to use them within the error message on failure
                let prev_hash_proposed = block.prev.as_ref().unwrap();
                let prev_hash_actual = self.blocks[block_num - 1].hash.as_ref().unwrap();

                if !(&block.prev == &self.blocks[block_num - 1].hash) {
                    return Err(format!("Block #{} is not connected to previous block (Hashes do \
                    not match. Should be `{}` but is `{}`)", block_num, prev_hash_proposed,
                                       prev_hash_actual).into());
                }
            }

            // Check if transactions are signed correctly
            for (transaction_num, transaction) in block.transactions.iter().enumerate() {

                // Careful! With that implementation an unsigned message will always
                // be valid! You may remove the first check to only accept signed transactions
                if transaction.is_signed() && !transaction.check_signature() {
                    return Err(format!("Transaction #{} for Block #{} has an invalid signature \
                    (Code: 4398239048)", transaction_num + 1, block_num + 1));
                }
            }
        }
        Ok(())
    }
}


impl WorldState for Blockchain {
    fn get_user_ids(&self) -> Vec<String> {
        self.accounts.keys().map(|s| s.clone()).collect()
    }

    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account> {
        self.accounts.get_mut(id)
    }

    fn get_account_by_id(&self, id: &String) -> Option<&Account> {
        self.accounts.get(id)
    }

    fn create_account(
        &mut self,
        id: String,
        account_type: AccountType,
    ) -> Result<(), &'static str> {
        return if !self.get_user_ids().contains(&id) {
            let acc = Account::new(account_type);
            self.accounts.insert(id, acc);
            Ok(())
        } else {
            Err("User already exists! (Code: 934823094)")
        };
    }
}
