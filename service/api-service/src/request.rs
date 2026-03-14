use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct CreateRideRequest {
    pub(crate) rider_id: String,
    pub(crate) pickup_lat: f64,
    pub(crate) pickup_lon: f64,
    pub(crate) dropoff_lat: f64,
    pub(crate) dropoff_lon: f64,
}
