use std::sync::Arc;

use tokio::sync::MutexGuard;

use axum::{http::StatusCode, Json};
use eternal_core::{block::Block, blockchain::Blockchain};
use serde_json::{json, Value};

pub fn get_blocks(bc: &Arc<Blockchain>) -> (StatusCode, Json<Vec<Block>>) {
    let blocks = &bc.blocks;
    (StatusCode::OK, Json(blocks.clone()))
}

pub fn get_block(bc: &mut MutexGuard<Blockchain>, hash: String) -> (StatusCode, Json<Block>) {
    let block = bc.get_block(hash);
    (StatusCode::OK, Json(block.clone()))
}

pub fn get_chain_height(bc: &mut MutexGuard<Blockchain>) -> (StatusCode, Json<Value>) {
    let height = bc.len();
    (
        StatusCode::OK,
        Json(json!({
            "height": height.clone()
        })),
    )
}
