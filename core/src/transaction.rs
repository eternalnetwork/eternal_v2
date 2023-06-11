use std::time::SystemTime;

use eternal_account::AccountType;
use eternal_vm::smart_contract::{self, SmartContract, SmartContractStanderd};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use eternal_vm::WorldState;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub nonce: u128,
    pub from: String,
    pub created_at: SystemTime,
    pub data: TransactionData,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum TransactionData {
    CreateUserAccount,
    ChangeStoreValue {
        key: String,
        value: String,
    },
    TransferToken {
        token: String,
        to: String,
        amount: u128,
    },
    Transfer {
        to: String,
        amount: u128,
    },
    MintTokens {
        receiver: String,
        amount: u128,
    },
    DeploySmartContract {
        publisher: String,
        #[serde(skip_serializing, skip_deserializing)]
        sc: Option<SmartContract>,
    },
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
    ) -> Result<String, &'static str> {
        if let Some(_account) = world_state.get_account_by_id(&self.from) {
            // Do some more checkups later on...
        } else {
            if !is_initial {
                return Err("Account does not exist (Code: 93482390)");
            }
        }

        return match &self.data {
            TransactionData::CreateUserAccount => {
                world_state.create_account(AccountType::User).unwrap();
                Ok("Created Account".to_string())
            }

            TransactionData::MintTokens { receiver, amount } => {
                if !is_initial {
                    return Err(
                        "Token creation is only available on initial creation (Code: 2394233)",
                    );
                }

                return if let Some(account) = world_state.get_account_by_id_mut(receiver) {
                    account.tokens += *amount;
                    Ok("Minted".to_string())
                } else {
                    Err("Receiver Account does not exist (Code: 23482309)")
                };
            }

            TransactionData::Transfer { to, amount } => {
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
                    return Ok("Trasnferd".to_string());
                } else {
                    return Err("Overspent or Arithmetic error (Code: 48239084203)");
                }
            }

            TransactionData::DeploySmartContract { publisher, sc } => {
                match sc.clone().unwrap().api {
                    smart_contract::SmartContractApi::ESC20 { mut publisher, .. } => {
                        publisher = publisher;
                        publisher
                    }
                };
                let contract_addr = &world_state
                    .create_smart_contact(sc.clone().unwrap())
                    .unwrap();
                for (_, account) in world_state.get_accounts() {
                    account
                        .store
                        .insert(contract_addr.clone(), (0 as u128).to_string());
                }

                let total_supply = match sc.clone().unwrap().api {
                    smart_contract::SmartContractApi::ESC20 { total_suply, .. } => total_suply,
                };
                let account = world_state.get_account_by_id_mut(publisher).unwrap();
                account
                    .store
                    .insert(contract_addr.clone(), total_supply.to_string());

                Ok(contract_addr.clone())
            }

            TransactionData::TransferToken { token, to, amount } => {
                let smart_contract = world_state.get_smart_contacts().get_mut(token);

                match smart_contract {
                    Some(sc) => {
                        if sc.r#type == SmartContractStanderd::ESC20 {
                            sc.execute_fn("transfer", vec![&self.from, to, &amount.to_string()]);
                        } else if sc.r#type == SmartContractStanderd::ESC721 {
                        } else {
                            return Err("Not a transferable assest");
                        }
                    }
                    None => return Err("Token does not exist"),
                }

                Ok("Token transfer success".to_string())
            }

            TransactionData::ChangeStoreValue { key, value } => {
                let acc = world_state.get_account_by_id_mut(&self.from).unwrap();

                if key.starts_with("etnl:") {
                    return Err("Can not change store related to a smart contract");
                }

                acc.store.insert(key.clone(), value.clone());

                Ok("Store updated".to_string())
            }
            // _ => Err("Unknown Transaction type (not implemented) (Code: 487289724389)"),
        };
    }

    /// Will calculate the hash using Blake2 hasher
    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();

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
