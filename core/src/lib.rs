use crate::account::{Account, AccountType};
pub use eternal_account as account;
pub mod block;
pub mod blockchain;
pub mod transaction;

pub trait WorldState {
    /// Will bring us all registered user ids
    fn get_user_ids(&self) -> Vec<String>;

    /// Will return an account given it id if is available (mutable)
    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account>;

    /// Will return an account given it id if is available
    fn get_account_by_id(&self, id: &String) -> Option<&Account>;

    /// Will add a new account
    fn create_account(&mut self, account_type: AccountType)
        -> Result<(), &'static str>;
}
