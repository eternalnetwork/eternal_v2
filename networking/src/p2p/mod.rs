use eternal_account::{Account, AccountType};
use libp2p::{floodsub::Topic, PeerId};
use once_cell::sync::Lazy;

pub static KEYS: Lazy<Account> = Lazy::new(|| Account::new(AccountType::Node));
pub static PEER_ID: Lazy<PeerId> =
    Lazy::new(|| PeerId::from_bytes(KEYS.public_key_bytes.as_slice()).unwrap());
pub static CHAIN_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("chains"));
pub static BLOCK_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("blocks"));

pub fn print_peer_id() {
    println!("{}", PEER_ID.to_base58())
}
