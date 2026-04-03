use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    ExchangeKind,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AmqpPublisher {
    channel: Arc<Mutex<Channel>>,
}

impl AmqpPublisher {
    pub async fn new(amqp_url: &str) -> Result<Self, lapin::Error> {
        let conn = Connection::connect(amqp_url, ConnectionProperties::default()).await?;
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
        Ok(Self {
            channel: Arc::new(Mutex::new(channel)),
        })
    }

    pub async fn publish(&self, routing_key: &str, payload: &[u8]) -> Result<(), lapin::Error> {
        let ch = self.channel.lock().await;
        ch.basic_publish(
            "polymove.events",
            routing_key,
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default()
                .with_content_type("application/json".into())
                .with_delivery_mode(2),
        )
        .await?
        .await?;
        Ok(())
    }
}
