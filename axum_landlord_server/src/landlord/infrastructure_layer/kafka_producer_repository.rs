use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::time::Duration;

pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> Self {
        // Initialize Kafka producer configuration
        let producer: FutureProducer = rdkafka::config::ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()
            .expect("Producer creation failed");

        KafkaProducer { producer }
    }

    pub async fn send_message(
        &self,
        topic: &str,
        key: &str,
        message: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let record = FutureRecord::to(topic)
            .key(key)
            .payload(message);

        // Send message to Kafka topic
        self.producer
            .send(record, Timeout::After(Duration::from_secs(1)))
            .await
            .map_err(|e| e.to_owned())
            .unwrap();
        Ok(())
    }
}
