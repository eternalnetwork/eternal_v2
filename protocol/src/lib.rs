mod accounts;
mod blockchain;

use std::{fmt, sync::Arc};

use axum::{extract::{Path}, routing::get, Router, Extension,};
use eternal_core::{blockchain::Blockchain};

pub struct EnpServer {
    pub router: Router,
    pub bc: Arc<Blockchain>,
}

#[derive(Debug, Default)]
pub enum RouteStatus {
    #[default] Active,
    Unavalible,
}

#[derive(Debug, Default)]
pub struct Routes {
    pub blockchain_getBlocks: RouteStatus,
    pub blockchain_getBlock_hash: RouteStatus,
    pub blockchain_getChainHeight: RouteStatus,
    pub account_getAccounts: RouteStatus,
    pub blockchain_getAccount_address: RouteStatus,
}

impl fmt::Display for Routes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "/blockchain/getBlocks: {}\n/blockchain/getBlock/<hash>: {}\n/blockhchain/getChainHeight: {}\n/accounts/getAccounts: {}\n/accounts/getAccount/<address>: {}", 
            self.blockchain_getBlocks, 
            self.blockchain_getBlock_hash, 
            self.blockchain_getChainHeight,
            self.account_getAccounts,
            self.blockchain_getAccount_address
        )
    }
}

impl fmt::Display for RouteStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        let text = match self {
            RouteStatus::Active => "Active",
            RouteStatus::Unavalible => "Unavalible",
        };

        write!(f, "{}", text)
    }
}

impl EnpServer {
    pub fn new(bc: Blockchain) -> Self {
        let router = Router::new();
        Self { router, bc: Arc::new(bc) }
    }

    pub async fn setup(&mut self) {
        let router: Router = Router::new();
        let router = router.route("/", get(|| async { "ENP (Eternal Network Protocol) v1.0" }));
        let router = router.route(
            "/blockchain/getBlocks",
            get(|Extension(bc): Extension<Arc<Blockchain>>| async move { 
                blockchain::get_blocks(&bc) 
            }),
        );
        // let router = router.route(
        //     "/blockchain/getBlock/:hash",
        //     get(|Path(hash): Path<String>| async move { blockchain::get_block(&mut bc, hash) }),
        // );
        // let router = router.route(
        //     "/blockchain/getChainHeight",
        //     get(|| async move { blockchain::get_chain_height(&mut self.bc.lock().await) }),
        // );
        // let router = router.route(
        //     "/accounts/getAccounts",
        //     get(|| async move { accounts::get_accounts(&mut self.bc.lock().await) }),
        // );
        // let router = router.route(
        //     "/accounts/getAccount/:address",
        //     get(|Path(address): Path<String>| async move { accounts::get_account(&mut self.bc.lock().await, address) }),
        // );
        self.router = router.layer(Extension(self.bc.clone())).clone()
    }

    pub async fn start(&self, host: (&str, u16)) {
        let info = Routes::default();
        // println!("{info}");
        axum::Server::bind(&format!("{}:{}", host.0, host.1).parse().unwrap())
            .serve(self.router.clone().into_make_service())
            .await
            .unwrap();
    }
}
