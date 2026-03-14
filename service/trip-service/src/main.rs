use kafka::consumer::KafkaConsumer;
use ride_events::{RideRequest, TopicName};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let consumer = KafkaConsumer::new("localhost:19092", "ride-request-debugger")?;
    consumer.subscribe(&[RideRequest::TOPIC.as_str()])?;

    loop {
        let evt = consumer.recv::<RideRequest>().await?;

        println!(
            "topic={} partition={} offset={} key={:?} ride_id={}",
            evt.topic,
            evt.partition,
            evt.offset,
            evt.key,
            evt.event.ride_id
        );

        consumer.commit_message(&evt)?;
    }
}