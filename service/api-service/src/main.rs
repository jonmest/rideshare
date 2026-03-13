use kafka::producer::{KafkaProducer, ProducerError};
use ride_events::{Position, RideRequest, google::protobuf::Timestamp};


#[tokio::main]
async fn main() -> Result<(), ProducerError> {
    let client = KafkaProducer::new("localhost:19092").map_err(ProducerError::Kafka)?;

    let msg1 = RideRequest {
                event_id: "1".to_string(),
                ride_id: "1".to_string(),
                rider_id: "1".to_string(),
                pickup: Some(Position {
                    lat: 3.0,
                    lon: 3.1
                }),
                dropoff: Some(Position {
                    lat: 3.0,
                    lon: 3.1
                }),
                requested_at: Some(Timestamp {
                    nanos: 13,
                    seconds: 12,
                }),
                meta: None,
            };

    client.publish(&msg1).await?;

    let msg2 = RideRequest {
                event_id: "2".to_string(),
                ride_id: "2".to_string(),
                rider_id: "2".to_string(),
                pickup: Some(Position {
                    lat: 3.0,
                    lon: 1.1
                }),
                dropoff: Some(Position {
                    lat: 2.0,
                    lon: 1.1
                }),
                requested_at: Some(Timestamp {
                    nanos: 19,
                    seconds: 20,
                }),
                meta: None,
            };

    client.publish(&msg2).await?;

    Ok(())
}