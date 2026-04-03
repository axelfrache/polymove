use crate::application::subscriber_service::SubscriberService;
use crate::ports::subscriber_repository::SubscriberRepository;
use futures::StreamExt;
use lapin::{
    options::*, types::FieldTable, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct StudentRegisteredEvent {
    pub student_id: String,
    pub name: String,
    pub domain: String,
}

#[derive(Debug, Deserialize)]
struct OfferCreatedEvent {
    pub title: String,
    pub city: String,
    pub domain: String,
}

async fn setup_channel(amqp_url: &str) -> Result<Channel, lapin::Error> {
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

    Ok(channel)
}

pub async fn setup_and_subscribe<R: SubscriberRepository + 'static>(
    amqp_url: &str,
    service: Arc<SubscriberService<R>>,
) -> Result<(), Box<dyn std::error::Error>> {
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

    channel
        .queue_declare(
            "laposte.student.registered",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            "laposte.student.registered",
            "polymove.events",
            "student.registered",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_declare(
            "laposte.offer.created",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            "laposte.offer.created",
            "polymove.events",
            "offer.created",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    spawn_student_consumer(channel.clone(), service.clone()).await?;
    spawn_offer_consumer(channel, service).await?;

    Ok(())
}

async fn spawn_student_consumer<R: SubscriberRepository + 'static>(
    channel: Channel,
    service: Arc<SubscriberService<R>>,
) -> Result<(), lapin::Error> {
    let mut consumer = channel
        .basic_consume(
            "laposte.student.registered",
            "laposte-student-consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    tokio::spawn(async move {
        tracing::info!("La Poste listening for student.registered events");
        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    match serde_json::from_slice::<StudentRegisteredEvent>(&delivery.data) {
                        Ok(event) => {
                            tracing::info!(
                                "Received student.registered: {} ({})",
                                event.name,
                                event.domain
                            );
                            if let Err(e) = service
                                .register_student(event.student_id, event.domain)
                                .await
                            {
                                tracing::error!("Failed to register student: {}", e);
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to deserialize student event: {}", e);
                        }
                    }
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(e) => {
                    tracing::error!("Consumer error: {}", e);
                }
            }
        }
    });

    Ok(())
}

async fn spawn_offer_consumer<R: SubscriberRepository + 'static>(
    channel: Channel,
    service: Arc<SubscriberService<R>>,
) -> Result<(), lapin::Error> {
    let mut consumer = channel
        .basic_consume(
            "laposte.offer.created",
            "laposte-offer-consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    tokio::spawn(async move {
        tracing::info!("La Poste listening for offer.created events");
        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    match serde_json::from_slice::<OfferCreatedEvent>(&delivery.data) {
                        Ok(event) => {
                            tracing::info!(
                                "Received offer.created: {} in {}",
                                event.title,
                                event.city
                            );
                            if let Err(e) = service
                                .send_offer_alert(&event.domain, &event.title, &event.city)
                                .await
                            {
                                tracing::error!("Failed to send offer alert: {}", e);
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to deserialize offer event: {}", e);
                        }
                    }
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(e) => {
                    tracing::error!("Consumer error: {}", e);
                }
            }
        }
    });

    Ok(())
}
