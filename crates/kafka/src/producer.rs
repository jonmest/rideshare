use prost::Message;
use rdkafka::{ClientConfig, error::KafkaError, producer::FutureProducer};
use ride_events::{ KafkaKey, TopicName};
use std::time::Duration;

use rdkafka::{
    producer::{ FutureRecord},
};


#[derive(Debug)]
pub enum ProducerError {
    Kafka(KafkaError),
    Serialization(prost::EncodeError),
}

impl From<prost::EncodeError> for ProducerError {
    fn from(err: prost::EncodeError) -> Self {
        Self::Serialization(err)
    }
}


pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(bootstrap_servers: &str) -> Result<Self, KafkaError> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self { producer })
    }

    pub async fn publish<T>(&self, event: &T) -> Result<(i32, i64), ProducerError>
    where
        T: Message + TopicName + KafkaKey,
    {
        let mut buf = Vec::new();
        event.encode(&mut buf)?;

        let (partition, offset) = self
            .producer
            .send(
                FutureRecord::to(T::TOPIC.as_str())
                    .payload(&buf)
                    .key(event.kafka_key()),
                Duration::from_secs(0),
            )
            .await
            .map_err(|(err, _msg)| ProducerError::Kafka(err))?;

        Ok((partition, offset))
    }
}