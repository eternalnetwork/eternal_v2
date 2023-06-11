pub mod smart_contract;
use std::collections::HashMap;

use eternal_account::{Account, AccountType};
use smart_contract::SmartContract as SC;

pub trait WorldState {
    /// Will bring us all registered user ids
    fn get_user_ids(&self) -> Vec<String>;

    /// Will return an account given it id if is available (mutable)
    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account>;

    /// Will return an account given it id if is available
    fn get_account_by_id(&self, id: &String) -> Option<&Account>;

    /// Will add a new account
    fn create_account(&mut self, account_type: AccountType) -> Result<String, &'static str>;

    fn get_smart_contact_ids(&self) -> Vec<String>;

    /// Will return an account given it id if is available (mutable)
    fn get_smart_contact_by_id_mut(&mut self, id: &String) -> Option<&mut SC>;

    /// Will return an account given it id if is available
    fn get_smart_contact_by_id(&self, id: &String) -> Option<&SC>;

    /// Will add a new account
    fn create_smart_contact(
        &mut self,
        account_type: SC,
    ) -> Result<String, &'static str>;

    fn get_accounts(&mut self) -> &mut HashMap<String, Account>;

    fn get_smart_contacts(&mut self) -> &mut HashMap<String, SC>;
}

pub trait SmartContract {
    fn deploy(&self) -> SC;
}