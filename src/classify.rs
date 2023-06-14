use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct ClassifyRequest {
    pub names: Vec<String>,
    pub labels: Vec<String>,
    pub blocks: Vec<Block>,
}

/// Skeleton type for pulling out the slot of each block.
#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    #[serde(with = "serde_utils::quoted_u64")]
    pub slot: u64,
    #[serde(flatten)]
    pub rest: HashMap<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BlockprintClassification {
    pub best_guess_single: String,
}
