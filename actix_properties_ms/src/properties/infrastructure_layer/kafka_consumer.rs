use log::{info, warn};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::Message;
use rdkafka::topic_partition_list::TopicPartitionList;
use prost::Message as ProstMessage;

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

type LoggingConsumer = StreamConsumer<CustomContext>;

#[derive(Clone, PartialEq, ::prost::Message)]
struct DataStruct {
  #[prost(string, tag = "1")]
  key: String,
  #[prost(string, tag = "2")]
  payload: String,
}

pub async fn consume_and_print(brokers: &str, group_id: &str, topic: &str) {
  let context = CustomContext;

  let consumer: LoggingConsumer = ClientConfig::new()
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

  loop {
    match consumer.recv().await {
      Err(e) => {
        warn!("Kafka error: {}", e);
        consumer.commit_consumer_state(CommitMode::Async).unwrap();
      }
      Ok(m) => {
        let payload = match m.payload_view::<[u8]>() {
          None => Err("No payload found".to_owned()),
          Some(Ok(bytes)) => {
            // Deserialize Protobuf message from bytes
            DataStruct::decode(bytes).map_err(|err| format!("Error deserializing Protobuf message: {}", err))
          }
          Some(Err(e)) => Err(format!("Error deserializing message payload: {:?}", e)),
        };

        match payload {
          Ok(data) => {
            println!(
              "key: '{:?}', payload: '{:?}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
              m.key(),
              data,
              m.topic(),
              m.partition(),
              m.offset(),
              m.timestamp()
            );
          }
          Err(err) => warn!("Error processing message: {}", err),
        }

        consumer.commit_message(&m, CommitMode::Async).unwrap();
      }
    };
  }
}

