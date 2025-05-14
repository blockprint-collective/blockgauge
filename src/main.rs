use crate::{
    accuracy::{AccuracyTracker, Summary},
    classify::{BlockprintClassification, ClassifyRequest},
    config::Config,
};
use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Router,
};
use clap::Parser;
use reqwest::Client;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::RwLock};

mod accuracy;
mod classify;
mod config;

pub struct Error {
    code: StatusCode,
    message: String,
}

impl Error {
    pub fn server_error(message: String) -> Self {
        Error {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}

/// Classify new blocks from blockdreamer and record them for future calls to `/accuracy`.
async fn classify(
    Extension(client): Extension<Client>,
    Extension(tracker): Extension<Arc<RwLock<AccuracyTracker>>>,
    Extension(conf): Extension<Arc<Config>>,
    Json(request): Json<ClassifyRequest>,
) -> Result<Json<Vec<Value>>, Error> {
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
        .map_err(|e| Error::server_error(format!("Lighthouse POST error: {e}")))?;

    if !block_rewards_res.status().is_success() {
        return Err(Error::server_error(format!(
            "error from Lighthouse: {}",
            block_rewards_res
                .text()
                .await
                .unwrap_or_else(|_| "<body garbled>".into())
        )));
    }

    let block_rewards: Vec<Value> = block_rewards_res.json().await.map_err(|e| {
        Error::server_error(format!("invalid JSON from Lighthouse block_rewards: {e}"))
    })?;

    // Classify block rewards with blockprint.
    println!("sending to blockprint");
    let classifications_res = client
        .post(format!("{}/classify/no_store", conf.blockprint_url))
        .json(&block_rewards)
        .send()
        .await
        .map_err(|e| Error::server_error(format!("Blockprint POST error: {e}")))?;

    if !classifications_res.status().is_success() {
        return Err(Error::server_error(format!(
            "error from blockprint: {}",
            classifications_res
                .text()
                .await
                .unwrap_or_else(|_| "<body garbled>".into())
        )));
    }

    let classifications: Vec<BlockprintClassification> = classifications_res
        .json()
        .await
        .map_err(|e| Error::server_error(format!("invalid JSON from blockprint: {e}")))?;

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
) -> Result<Json<Summary>, Error> {
    if let Some(summary) = tracker.read().await.summarise() {
        Ok(Json(summary))
    } else {
        Err(Error::server_error("error computing accuracy".into()))
    }
}

#[tokio::main]
async fn main() {
    let conf = Arc::new(Config::parse());
    eprintln!("Starting up with config: {conf:#?}");

    let http_client = Client::new();

    let tracker = Arc::new(RwLock::new(AccuracyTracker::default()));

    let app = Router::new()
        .route("/classify", post(classify))
        .route("/accuracy", get(accuracy))
        .layer(Extension(http_client))
        .layer(Extension(tracker))
        .layer(Extension(conf.clone()));

    let bind_futures = conf.listen_address.iter().map(|listen_address| async {
        let socket_addr = SocketAddr::new(listen_address.clone(), conf.port);
        let listener = TcpListener::bind(socket_addr).await?;
        Ok::<_, std::io::Error>(axum::serve(listener, app.clone()))
    });

    futures::future::join_all(bind_futures)
        .await
        .into_iter()
        .map(Result::unwrap)
        .for_each(drop);
}
