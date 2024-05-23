use log::{info, warn};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::Headers;
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::Message;
use serde::Deserialize;

struct CustomContext;

impl ClientContext for CustomContext {
    const ENABLE_REFRESH_OAUTH_TOKEN: bool = false;
}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

#[derive(Deserialize)]
struct DataStruct {
    key: String,
    payload: String,
}


pub struct CustomConsumer {
    inner: StreamConsumer<CustomContext>,
}

impl CustomConsumer {
    pub fn new(brokers: &str, group_id: &str, topic: &str) -> CustomConsumer {
        let context = CustomContext;
        let consumer: StreamConsumer<CustomContext> = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .expect("Consumer creation failed");

        consumer
            .subscribe(&[topic])
            .expect("Can't subscribe to specified topic");

        CustomConsumer { inner: consumer }
    }

    pub async fn consume_loop(&self) {
        loop {
            match self.inner.recv().await {
                Err(e) => {
                    warn!("Kafka error: {}", e);
                    self.inner
                        .commit_consumer_state(CommitMode::Async)
                        .unwrap();
                }
                Ok(m) => {
                    if let Some(headers) = m.headers() {
                        for header in headers.iter() {
                            println!("Header {:#?}: {:?}", header.key, header.value);
                        }
                    }
                    self.inner.commit_message(&m, CommitMode::Async).unwrap();
                }
            };
        }
    }
}

pub async fn k_consumer() {
    let brokers = "localhost:9092";
    let group_id: &str = "consumer-group-a";
    let topic = "register_landlords";
    let consumer = CustomConsumer::new(brokers, group_id, topic);
    consumer.consume_loop().await;
}
