use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
pub struct ClassifyRequest {
    pub names: Vec<String>,
    pub labels: Vec<String>,
    pub blocks: Vec<Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BlockprintClassification {
    pub best_guess_single: String,
}
