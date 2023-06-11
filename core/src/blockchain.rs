use eternal_account::{Account, AccountType};
use eternal_vm::smart_contract::SmartContract;
use serde::{Deserialize, Serialize};

use crate::{block::Block, transaction::Transaction};
use eternal_vm::WorldState;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    #[serde(skip_serializing, skip_deserializing)]
    pub smart_contracts: HashMap<String, SmartContract>,
    pub accounts: HashMap<String, Account>,
    pub temp: Vec<String>,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let accounts = HashMap::new();
        let scs = HashMap::new();
        Self {
            blocks: Vec::new(),
            accounts,
            temp: vec![],
            smart_contracts: scs,
            pending_transactions: Vec::new(),
        }
    }

    pub fn get_bob_alice(&mut self) -> (String, String) {
        let mut alice = Account::new(AccountType::User);
        let mut bob = Account::new(AccountType::User);
        bob.tokens = 100_000;
        alice.tokens = 100_00;
        println!("\n{}\n{}", bob.generate_adress(), alice.generate_adress());

        let alice_addr = alice.generate_adress().clone();
        let bob_arr = bob.generate_adress().clone();

        self.accounts.insert(alice.generate_adress(), alice);
        self.accounts.insert(bob.generate_adress(), bob);

        (bob_arr, alice_addr)
    }

    pub fn get_block(&self, hash: String) -> Block {
        let binding = self.blocks.clone().to_owned();
        let block: &Block = binding
            .iter()
            .find(|b| b.hash == Some(hash.clone()))
            .unwrap();

        let block = block.clone();

        block
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
            inside the block! (Code: 9482930)"
                .into());
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

            match transaction.execute(self, &is_genesis) {
                Err(err) => {
                    self.accounts = old_state;

                    return Err(format!(
                        "Could not execute transaction {} due to `{}`. Rolling back \
                    (Code: 38203984)",
                        i + 1,
                        err
                    ));
                }
                Ok(out) => self.temp.push(out),
            }
        }

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
                return Err(format!(
                    "Stored hash for Block #{} \
                    does not match calculated hash (Code: 665234234)",
                    block_num + 1
                )
                .into());
            }

            // Check previous black hash points to actual previous block
            if block_num == 0 {
                // Genesis block should point to nowhere
                if block.prev.is_some() {
                    return Err("The genesis block has a previous hash set which \
                     it shouldn't Code :394823098"
                        .into());
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
                    return Err(format!(
                        "Block #{} is not connected to previous block (Hashes do \
                    not match. Should be `{}` but is `{}`)",
                        block_num, prev_hash_proposed, prev_hash_actual
                    )
                    .into());
                }
            }

            // Check if transactions are signed correctly
            for (transaction_num, transaction) in block.transactions.iter().enumerate() {
                // Careful! With that implementation an unsigned message will always
                // be valid! You may remove the first check to only accept signed transactions
                if transaction.is_signed() && !transaction.check_signature() {
                    return Err(format!(
                        "Transaction #{} for Block #{} has an invalid signature \
                    (Code: 4398239048)",
                        transaction_num + 1,
                        block_num + 1
                    ));
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

    fn create_account(&mut self, account_type: AccountType) -> Result<String, &'static str> {
        let acc = Account::new(account_type);
        let address = acc.generate_adress();
        return if !self.get_user_ids().contains(&address) {
            self.accounts.insert(address.clone(), acc);
            Ok(address.clone())
        } else {
            Err("User already exists! (Code: 934823094)")
        };
    }

    fn get_smart_contact_ids(&self) -> Vec<String> {
        self.smart_contracts.keys().map(|s| s.clone()).collect()
    }

    fn get_smart_contact_by_id_mut(&mut self, id: &String) -> Option<&mut SmartContract> {
        self.smart_contracts.get_mut(id)
    }

    fn get_smart_contact_by_id(&self, id: &String) -> Option<&SmartContract> {
        self.smart_contracts.get(id)
    }

    fn create_smart_contact(
        &mut self,
        smart_contract: SmartContract,
    ) -> Result<String, &'static str> {
        let address = self.create_account(AccountType::SmartContract).unwrap();
        return if !self.get_smart_contact_ids().contains(&address) {
            self.smart_contracts.insert(address.clone(), smart_contract);
            Ok(address)
        } else {
            Err("User Contract exists! (Code: 934823094)")
        };
    }

    fn get_accounts(&mut self) -> &mut HashMap<String, Account> {
        &mut self.accounts
    }

    fn get_smart_contacts(&mut self) -> &mut HashMap<String, SmartContract> {
        &mut self.smart_contracts
    }
}
