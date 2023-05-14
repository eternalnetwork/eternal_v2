use std::collections::HashMap;

use axum::{http::StatusCode, Json};
use eternal_core::{account::Account, blockchain::Blockchain, WorldState};
use tokio::sync::MutexGuard;

pub fn get_accounts(
    bc: &mut MutexGuard<Blockchain>,
) -> (StatusCode, Json<HashMap<String, Account>>) {
    let accounts = bc.accounts.clone();
    (StatusCode::OK, Json(accounts.clone()))
}

pub fn get_account(
    bc: &mut MutexGuard<Blockchain>,
    address: String,
) -> (StatusCode, Json<Account>) {
    let account = bc.get_account_by_id(&address).unwrap();
    (StatusCode::OK, Json(account.clone()))
}
