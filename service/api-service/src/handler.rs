use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use ride_events::{DriverAvailability, DriverLocationUpdate, DriverStatus, EventMetadata, Position, RideRequest, google::protobuf::Timestamp};
use tracing::{error, info};

use crate::{AppState, request::{CreateRideRequest, UpdateDriverAvailabilityRequest, UpdateDriverLocationRequest}};

pub(crate) async fn update_driver_availability(
    axum::extract::Path(driver_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateDriverAvailabilityRequest>
) -> Result<impl IntoResponse, StatusCode> {
    let event_id = uuid::Uuid::new_v4().to_string();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let status = match payload.status.as_str() {
        "available" => Result::Ok(DriverStatus::Available),
        "unavailable" => Result::Ok(DriverStatus::Unavailable),
        "unspecified" => Result::Ok(DriverStatus::Unspecified),
        _ => Result::Err(StatusCode::BAD_REQUEST)
    }?;
    
    let message = DriverAvailability {
        event_id,
        driver_id,
        updated_at: Some(Timestamp { seconds: now, nanos: 0 }),
        status: status.to_int(),
        meta: None,
    };

    let (partition, offset) = state
        .kafka_producer
        .publish(&message)
        .await
        .map_err(|e| {
            error!(error = ?e, "failed to publish driver availability update");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!(partition, offset, "published driver availability update");
    Ok((StatusCode::CREATED, Json(payload)))
}

pub(crate) async fn update_driver_location(
    axum::extract::Path(driver_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateDriverLocationRequest>
) -> Result<impl IntoResponse, StatusCode> {
    let event_id = uuid::Uuid::new_v4().to_string();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    
    let message = DriverLocationUpdate {
        event_id,
        driver_id,
        lat: payload.lat,
        lon: payload.lon,
        recorded_at: Some(Timestamp { seconds: now, nanos: 0 }),
        meta: None,
    };

    let (partition, offset) = state
        .kafka_producer
        .publish(&message)
        .await
        .map_err(|e| {
            error!(error = ?e, "failed to publish driver location update");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!(partition, offset, "published driver location update");
    Ok((StatusCode::CREATED, Json(payload)))
}

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
