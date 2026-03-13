use std::{time::Duration};

use prost::Message;
use rdkafka::{ClientConfig, message::{Header, OwnedHeaders}, producer::{FutureProducer, FutureRecord}};
use ride_events::RideRequest;

async fn publish() {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "localhost:19092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let futures = (0..5)
        .map(|i| async move {
            let mut buf = Vec::new();
            let mut msg = RideRequest::default();
            msg.event_id = "RandomID".to_string();
            match msg.encode(&mut buf) {
                Ok(_) => {
                    let delivery_status = producer
                        .send(
                            FutureRecord::to("ride-requests")
                                .payload(&buf)
                                .key(&format!("Key {}", i))
                                .headers(OwnedHeaders::new().insert(Header {
                                    key: "header_key",
                                    value: Some("header_value"),
                                })),
                            Duration::from_secs(0),
                        )
                        .await;
                    
                    // This will be executed when the result is received.
                    println!("Delivery status for message {} received", i);
                    delivery_status
                },
                Err(_) => panic!("Failed")
            }
            
            
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received.
    for future in futures {
        println!("Future completed. Result: {:?}", future.await);
    }
}

#[tokio::main]
async fn main() {
    publish().await;
}