use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;

use crate::models::OrderShippedMessage;
use crate::notifier;

pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub group_id: String,
    pub topic: String,
}

pub async fn run(config: KafkaConfig) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &config.bootstrap_servers)
        .set("group.id", &config.group_id)
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Failed to create Kafka consumer");

    consumer
        .subscribe(&[&config.topic])
        .expect("Failed to subscribe to topic");

    tracing::info!(topic = %config.topic, "Kafka consumer started");

    loop {
        match consumer.recv().await {
            Err(e) => tracing::warn!("Kafka error: {}", e),
            Ok(msg) => {
                let payload = match msg.payload_view::<str>() {
                    Some(Ok(s)) => s,
                    _ => {
                        tracing::warn!("Received non-UTF8 message, skipping");
                        continue;
                    }
                };

                match serde_json::from_str::<OrderShippedMessage>(payload) {
                    Ok(event) => {
                        tracing::info!(
                            order_id = %event.order_id,
                            user_id = %event.user_id,
                            tracking = %event.tracking_number,
                            "Received order shipped event"
                        );
                        notifier::notify(&event.user_id, &event.order_id, &event.tracking_number);
                        consumer.commit_message(&msg, CommitMode::Async).ok();
                    }
                    Err(e) => {
                        tracing::warn!("Failed to deserialize message: {}: {}", e, payload);
                    }
                }
            }
        }
    }
}
