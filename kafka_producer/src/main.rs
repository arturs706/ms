use log::info;
use prost::{EncodeError, Message};
use rdkafka::config::ClientConfig;
use rdkafka::error::RDKafkaErrorCode;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use std::time::Duration; // Import RDKafkaErrorCode from rdkafka::error

// Define your Protobuf message structure
#[derive(Clone, PartialEq, ::prost::Message)]
struct DataStruct {
    #[prost(string, tag = "1")]
    key: String,
    #[prost(string, tag = "2")]
    payload: String,
}

async fn produce(key_acc: &str, payload: &str) -> Result<(), rdkafka::error::KafkaError> {
    let brokers = "localhost:19092";
    let topic_name = "chat-room";
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // Create an instance of your data struct
    let data = DataStruct {
        key: key_acc.to_string(),
        payload: payload.to_string(),
    };

    // Serialize your data to Protobuf binary format
    let mut buf = Vec::new();
    if let Err(err) = data.encode(&mut buf) {
        // Map EncodeError to an appropriate Kafka error code
        let kafka_error_code = RDKafkaErrorCode::MessageSizeTooLarge; // or any other appropriate error code
        return Err(rdkafka::error::KafkaError::MessageProduction(
            kafka_error_code,
        ));
    }

    let record = FutureRecord::to(topic_name)
        .payload(&buf)
        .key("key".as_bytes());

    let (partition, offset) = producer
        .send(record, Duration::from_secs(0))
        .await
        .expect("Message failed to be produced");

    info!(
        "Published message at topic '{}' partition '{}' offset '{}'",
        topic_name, partition, offset
    );
    producer
        .flush(Duration::from_secs(1))
        .expect("Flushing failed");
    Ok(())
}

#[tokio::main]
async fn main() {
    let key_acc = "test-key";
    let payload = "payload";

    match produce(key_acc, payload).await {
        Ok(_) => info!("Message produced successfully"),
        Err(e) => info!("Failed to produce message: {:?}", e),
    }
}
