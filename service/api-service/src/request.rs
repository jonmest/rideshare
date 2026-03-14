use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct CreateRideRequest {
    pub(crate) rider_id: String,
    pub(crate) pickup_lat: f64,
    pub(crate) pickup_lon: f64,
    pub(crate) dropoff_lat: f64,
    pub(crate) dropoff_lon: f64,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct UpdateDriverLocationRequest {
    pub(crate) driver_id: String,
    pub(crate) lon: f64,
    pub(crate) lat: f64,
    pub(crate) update_time: i64
}

#[derive(Deserialize, Serialize)]
pub(crate) struct UpdateDriverAvailabilityRequest {
    pub(crate) driver_id: String,
    pub(crate) update_time: i64,
    pub(crate) status: String,
}