use std::time::SystemTime;

use blake2::{Blake2s256, Digest};
use eternal_account::AccountType;
use serde::{Deserialize, Serialize};

use crate::WorldState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub nonce: u128,
    pub from: String,
    pub created_at: SystemTime,
    pub data: TransactionData,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionData {
    CreateUserAccount(String),
    ChangeStoreValue { key: String, value: String },
    TransferToken { to: String, amount: u128 },
    MintTokens { receiver: String, amount: u128 },
}

impl Transaction {
    pub fn new(from: String, transaction_data: TransactionData, nonce: u128) -> Self {
        Self {
            from,
            nonce,
            data: transaction_data,
            created_at: SystemTime::now(),
            signature: None,
        }
    }

    pub fn execute<T: WorldState>(
        &self,
        world_state: &mut T,
        is_initial: &bool,
    ) -> Result<(), &'static str> {
        if let Some(_account) = world_state.get_account_by_id(&self.from) {
            // Do some more checkups later on...
        } else {
            if !is_initial {
                return Err("Account does not exist (Code: 93482390)");
            }
        }

        return match &self.data {
            TransactionData::CreateUserAccount(account) => {
                world_state.create_account(account.into(), AccountType::User)
            }

            TransactionData::MintTokens { receiver, amount } => {
                if !is_initial {
                    return Err(
                        "Token creation is only available on initial creation (Code: 2394233)",
                    );
                }

                return if let Some(account) = world_state.get_account_by_id_mut(receiver) {
                    account.tokens += *amount;
                    Ok(())
                } else {
                    Err("Receiver Account does not exist (Code: 23482309)")
                };
            }

            TransactionData::TransferToken { to, amount } => {
                let recv_tokens: u128;
                let sender_tokens: u128;

                if let Some(recv) = world_state.get_account_by_id_mut(to) {
                    // Be extra careful here, even in the genesis block the sender account has to exist
                    recv_tokens = recv.tokens;
                } else {
                    return Err("Receiver Account does not exist! (Code: 3242342380)");
                }

                if let Some(sender) = world_state.get_account_by_id_mut(&self.from) {
                    sender_tokens = sender.tokens;
                } else {
                    return Err("That account does not exist! (Code: 23423923)");
                }

                let balance_recv_new = recv_tokens.checked_add(*amount);
                let balance_sender_new = sender_tokens.checked_sub(*amount);

                if balance_recv_new.is_some() && balance_sender_new.is_some() {
                    world_state
                        .get_account_by_id_mut(&self.from)
                        .unwrap()
                        .tokens = balance_sender_new.unwrap();
                    world_state.get_account_by_id_mut(&to).unwrap().tokens =
                        balance_recv_new.unwrap();
                    return Ok(());
                } else {
                    return Err("Overspent or Arithmetic error (Code: 48239084203)");
                }
            }

            _ => Err("Unknown Transaction type (not implemented) (Code: 487289724389)"),
        };
    }

    /// Will calculate the hash using Blake2 hasher
    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2s256::new();

        let transaction_as_string = format!(
            "{:?}",
            (&self.created_at, &self.data, &self.from, &self.nonce)
        );

        hasher.update(&transaction_as_string);

        let hash: &[u8] = &*hasher.finalize();

        return Vec::from(hash);
    }

    pub fn check_signature(&self) -> bool {
        if !(self.is_signed()) {
            return false;
        }

        // check signature
        false
    }

    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }
}
