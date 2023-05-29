use crate::{
    accuracy::{AccuracyTracker, Summary},
    classify::{BlockprintClassification, ClassifyRequest},
    config::Config,
};
use axum::{
    extract::Json,
    response::Result,
    routing::{get, post},
    Extension, Router,
};
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

mod accuracy;
mod classify;
mod config;

/// Classify new blocks from blockdreamer and record them for future calls to `/accuracy`.
async fn classify(
    Json(request): Json<ClassifyRequest>,
    Extension(client): Extension<Client>,
    Extension(tracker): Extension<Arc<RwLock<AccuracyTracker>>>,
    Extension(conf): Extension<Arc<Config>>,
) -> Result<Json<Vec<Value>>> {
    // Send blocks to Lighthouse.
    println!("posting blocks to Lighthouse");
    let block_rewards_res = client
        .post(format!(
            "{}/lighthouse/analysis/block_rewards",
            conf.lighthouse_url
        ))
        .json(&request.blocks)
        .send()
        .await
        .map_err(|e| format!("Lighthouse POST error: {e}"))?;

    if !block_rewards_res.status().is_success() {
        return Err(format!(
            "error from Lighthouse: {}",
            block_rewards_res
                .text()
                .await
                .unwrap_or_else(|_| "<body garbled>".into())
        )
        .into());
    }

    let block_rewards: Vec<Value> = block_rewards_res
        .json()
        .await
        .map_err(|e| format!("invalid JSON from Lighthouse block_rewards: {e}"))?;

    // Classify block rewards with blockprint.
    println!("sending to blockprint");
    let classifications_res = client
        .post(format!("{}/classify/no_store", conf.blockprint_url))
        .json(&block_rewards)
        .send()
        .await
        .map_err(|e| format!("Blockprint POST error: {e}"))?;

    if !classifications_res.status().is_success() {
        return Err(format!(
            "error from blockprint: {}",
            classifications_res
                .text()
                .await
                .unwrap_or_else(|_| "<body garbled>".into())
        )
        .into());
    }

    let classifications: Vec<BlockprintClassification> = classifications_res
        .json()
        .await
        .map_err(|e| format!("invalid JSON from blockprint: {e}"))?;

    // Record blockprint's accuracy.
    let mut tracker_guard = tracker.write().await;
    for ((id, true_label), (classified_as, block)) in request
        .names
        .into_iter()
        .zip(request.labels)
        .zip(classifications.into_iter().zip(request.blocks.iter()))
    {
        tracker_guard.record_block(id, true_label, classified_as.best_guess_single, block.slot);
    }

    // Return the unmodified block rewards.
    println!("success");
    Ok(Json(block_rewards))
}

/// Return statistics about classified blocks.
async fn accuracy(
    Extension(tracker): Extension<Arc<RwLock<AccuracyTracker>>>,
) -> Result<Json<Summary>> {
    if let Some(summary) = tracker.read().await.summarise() {
        Ok(Json(summary))
    } else {
        Err("error computing accuracy".into())
    }
}

#[tokio::main]
async fn main() {
    let conf = Arc::new(Config {
        lighthouse_url: "http://localhost:5052".into(),
        blockprint_url: "http://localhost:8001".into(),
    });

    let http_client = Client::new();

    let tracker = Arc::new(RwLock::new(AccuracyTracker::default()));

    let app = Router::new()
        .route("/classify", post(classify))
        .route("/accuracy", get(accuracy))
        .layer(Extension(http_client))
        .layer(Extension(tracker))
        .layer(Extension(conf));

    axum::Server::bind(&"127.0.0.1:8002".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
