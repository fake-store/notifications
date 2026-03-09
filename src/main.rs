mod kafka;
mod models;
mod notifier;
mod users_client;

use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let bootstrap_servers =
        env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap_or_else(|_| "localhost:9091".to_string());
    let group_id =
        env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "notifications-service".to_string());
    let topic =
        env::var("KAFKA_TOPIC").unwrap_or_else(|_| "shipping.order.shipped".to_string());

    tracing::info!("Starting notifications service");

    kafka::run(kafka::KafkaConfig {
        bootstrap_servers,
        group_id,
        topic,
    })
    .await;
}
