use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use ride_events::{EventMetadata, Position, RideRequest};
use tracing::{error, info};

use crate::{AppState, request::CreateRideRequest};

pub(crate) async fn create_ride(
    State(state): State<AppState>,
    Json(payload): Json<CreateRideRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let ride_id = uuid::Uuid::new_v4().to_string();
    let event_id = uuid::Uuid::new_v4().to_string();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let message = RideRequest {
        event_id: event_id.clone(),
        ride_id,
        rider_id: payload.rider_id.clone(),
        pickup: Some(Position {
            lat: payload.pickup_lat,
            lon: payload.pickup_lon,
        }),
        dropoff: Some(Position {
            lat: payload.dropoff_lat,
            lon: payload.dropoff_lon,
        }),
        requested_at: None,
        meta: Some(EventMetadata {
            event_id,
            occurred_at_ms: now,
            producer: "api-service".into(),
        }),
    };

    let (partition, offset) = state
        .kafka_producer
        .publish(&message)
        .await
        .map_err(|e| {
            error!(error = ?e, "failed to publish ride request");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!(partition, offset, "published ride request");

    Ok((StatusCode::CREATED, Json(payload)))
}
