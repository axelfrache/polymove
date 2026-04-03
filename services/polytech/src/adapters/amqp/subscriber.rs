use crate::application::notification_service::NotificationService;
use crate::ports::student_repository::StudentRepository;
use futures::StreamExt;
use lapin::{
    Channel, Connection, ConnectionProperties, ExchangeKind, options::*, types::FieldTable,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct OfferCreatedEvent {
    id: String,
    city: String,
    domain: String,
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

pub async fn start_offer_subscriber<R: StudentRepository + 'static>(
    amqp_url: &str,
    notification_service: Arc<NotificationService>,
    student_repository: Arc<R>,
) {
    match setup_offer_subscriber(amqp_url, notification_service, student_repository).await {
        Ok(_) => tracing::info!("Polytech offer.created subscriber started"),
        Err(e) => tracing::error!("Failed to start offer.created subscriber: {}", e),
    }
}

async fn setup_offer_subscriber<R: StudentRepository + 'static>(
    amqp_url: &str,
    notification_service: Arc<NotificationService>,
    student_repository: Arc<R>,
) -> Result<(), lapin::Error> {
    let channel = setup_channel(amqp_url).await?;

    channel
        .queue_declare(
            "polytech.offer.created",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            "polytech.offer.created",
            "polymove.events",
            "offer.created",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            "polytech.offer.created",
            "polytech-offer-consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    tokio::spawn(async move {
        while let Some(delivery_result) = consumer.next().await {
            match delivery_result {
                Ok(delivery) => {
                    match serde_json::from_slice::<OfferCreatedEvent>(&delivery.data) {
                        Ok(event) => {
                            tracing::info!(
                                "Polytech received offer.created: {} (domain: {})",
                                event.id,
                                event.domain
                            );

                            // Get all students in the matching domain
                            match student_repository.list_by_domain(&event.domain).await {
                                Ok(students) => {
                                    for student in students {
                                        let message = format!(
                                            "New offer available in {} for domain {}",
                                            event.city, event.domain
                                        );
                                        if let Err(e) = notification_service
                                            .create_notification(
                                                student.id,
                                                event.id.clone(),
                                                message,
                                            )
                                            .await
                                        {
                                            tracing::error!(
                                                "Failed to create notification for student {}: {}",
                                                student.id,
                                                e
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::error!(
                                        "Failed to list students for domain {}: {}",
                                        event.domain,
                                        e
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to deserialize offer.created event: {}", e);
                        }
                    }
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(e) => {
                    tracing::error!("Offer subscriber error: {}", e);
                }
            }
        }
    });

    Ok(())
}
