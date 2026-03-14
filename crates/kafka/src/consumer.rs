use std::{error::Error, fmt};

use prost::Message;
use rdkafka::{ClientConfig, Message as KafkaMessage, Offset, TopicPartitionList, consumer::{CommitMode, Consumer, StreamConsumer}, error::KafkaError};


#[derive(Debug)]
pub enum ConsumerError {
    Kafka(KafkaError),
    MissingPayload,
    Deserialization(prost::DecodeError),
}

impl From<KafkaError> for ConsumerError {
    fn from(err: KafkaError) -> Self {
        Self::Kafka(err)
    }
}

impl From<prost::DecodeError> for ConsumerError {
    fn from(err: prost::DecodeError) -> Self {
        Self::Deserialization(err)
    }
}

impl fmt::Display for ConsumerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsumerError::Kafka(e) => write!(f, "Kafka error: {}", e),
            ConsumerError::MissingPayload => write!(f, "Missing payload"),
            ConsumerError::Deserialization(e) => write!(f, "Decode error: {}", e),
        }
    }
}

impl Error for ConsumerError {}

pub struct ConsumedEvent<T> {
    pub event: T,
    pub key: Option<String>,
    pub partition: i32,
    pub offset: i64,
    pub topic: String,
}

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new(
        bootstrap_servers: &str,
        group_id: &str,
    ) -> Result<Self, KafkaError> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers)
            .set("group.id", group_id)
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "earliest")
            .create()?;

        Ok(Self { consumer })
    }

    pub fn subscribe(&self, topics: &[&str]) -> Result<(), KafkaError> {
        self.consumer.subscribe(topics)
    }

    pub async fn recv<T>(&self) -> Result<ConsumedEvent<T>, ConsumerError>
    where
        T: Message + Default,
    {
        let msg = self.consumer.recv().await?;

        let payload = msg.payload().ok_or(ConsumerError::MissingPayload)?;
        let event = T::decode(payload)?;

        let key = msg
            .key()
            .map(|k| String::from_utf8_lossy(k).into_owned());

        Ok(ConsumedEvent {
            event,
            key,
            partition: msg.partition(),
            offset: msg.offset(),
            topic: msg.topic().to_string(),
        })
    }

    pub fn commit_offset(
        &self,
        topic: &str,
        partition: i32,
        processed_offset: i64,
    ) -> Result<(), KafkaError> {
        let mut tpl = TopicPartitionList::new();

        tpl.add_partition_offset(
            topic,
            partition,
            Offset::Offset(processed_offset + 1),
        )?;

        self.consumer.commit(&tpl, CommitMode::Sync)
    }

    pub fn commit_message<T>(&self, event: &ConsumedEvent<T>) -> Result<(), KafkaError> {
        self.commit_offset(&event.topic, event.partition, event.offset)
    }
}