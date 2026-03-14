use std::sync::Arc;

use axum::{Router, routing::post};
use kafka::producer::{KafkaProducer, ProducerError};
use tracing::info;

mod handler;
mod request;

#[derive(Clone)]
struct AppState {
    kafka_producer: Arc<KafkaProducer>,
}

#[tokio::main]
async fn main() -> Result<(), ProducerError> {
    tracing_subscriber::fmt::init();

    let kafka_producer = KafkaProducer::new("localhost:19092").map_err(ProducerError::Kafka)?;
    let state = AppState {
        kafka_producer: Arc::new(kafka_producer),
    };

    let app = Router::new()
        .route("/rides", post(handler::create_ride))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
