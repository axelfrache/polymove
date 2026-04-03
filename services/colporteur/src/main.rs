use chrono::Utc;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties, ExchangeKind,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct News {
    id: String,
    name: String,
    source: String,
    date: String,
    tags: Vec<String>,
    city: String,
    country: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let amqp_url = std::env::var("AMQP_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@127.0.0.1:5672/%2f".to_string());

    tracing::info!("Connecting to RabbitMQ at {}", amqp_url);

    let conn = Connection::connect(&amqp_url, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    channel
        .exchange_declare(
            "polymove.events",
            ExchangeKind::Topic,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    tracing::info!("Connected to RabbitMQ, exchange declared");

    let cities = vec!["Paris", "Nice", "Lyon", "Marseille", "Bordeaux", "Toulouse"];
    let sources = vec!["Le Monde", "Nice Matin", "AFP", "Reuters", "TechCrunch"];
    let tags_list = vec![
        "innovation",
        "culture",
        "healthcare",
        "entertainment",
        "crisis",
        "crime",
        "disaster",
    ];

    for i in 0..10 {
        let city = cities[rand::thread_rng().gen_range(0..cities.len())];
        let tag = tags_list[rand::thread_rng().gen_range(0..tags_list.len())];
        let source = sources[rand::thread_rng().gen_range(0..sources.len())].to_string();

        let news = News {
            id: Uuid::new_v4().to_string(),
            name: format!("Breaking News #{} in {}", i, city),
            source,
            date: Utc::now().to_rfc3339(),
            tags: vec![tag.to_string()],
            city: city.to_string(),
            country: "France".to_string(),
        };

        tracing::info!("Publishing news: {} ({}) to {}", news.name, tag, city);

        let payload = serde_json::to_vec(&news)?;

        match channel
            .basic_publish(
                "polymove.events",
                "news.created",
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default()
                    .with_content_type("application/json".into())
                    .with_delivery_mode(2),
            )
            .await
        {
            Ok(confirm) => {
                let _ = confirm.await;
                tracing::info!("Published news: {} (ID: {})", news.name, news.id);
            }
            Err(e) => {
                tracing::error!("Failed to publish news: {}", e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
