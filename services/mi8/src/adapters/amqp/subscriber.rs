use crate::application::news_service::NewsService;
use crate::domain::model::News;
use crate::domain::ports::news_repository::NewsRepository;
use futures::StreamExt;
use lapin::{
    options::*, types::FieldTable, Channel, Connection, ConnectionProperties, ExchangeKind,
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

pub async fn start_subscribers<R: NewsRepository + 'static>(
    amqp_url: &str,
    service: Arc<NewsService<R>>,
) {
    // Subscriber for news.created
    match setup_news_subscriber(amqp_url, service.clone()).await {
        Ok(_) => tracing::info!("MI8 news.created subscriber started"),
        Err(e) => tracing::error!("Failed to start news.created subscriber: {}", e),
    }

    // Subscriber for offer.created
    match setup_offer_subscriber(amqp_url, service).await {
        Ok(_) => tracing::info!("MI8 offer.created subscriber started"),
        Err(e) => tracing::error!("Failed to start offer.created subscriber: {}", e),
    }
}

async fn setup_news_subscriber<R: NewsRepository + 'static>(
    amqp_url: &str,
    service: Arc<NewsService<R>>,
) -> Result<(), lapin::Error> {
    let channel = setup_channel(amqp_url).await?;

    channel
        .queue_declare(
            "mi8.news.created",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            "mi8.news.created",
            "polymove.events",
            "news.created",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            "mi8.news.created",
            "mi8-news-consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    tokio::spawn(async move {
        while let Some(delivery_result) = consumer.next().await {
            match delivery_result {
                Ok(delivery) => {
                    match serde_json::from_slice::<News>(&delivery.data) {
                        Ok(news) => {
                            tracing::info!("MI8 received news.created: {} in {}", news.name, news.city);
                            if let Err(e) = service.create_news(news).await {
                                tracing::error!("Failed to process news: {}", e);
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to deserialize news: {}", e);
                        }
                    }
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(e) => {
                    tracing::error!("News consumer error: {}", e);
                }
            }
        }
    });

    Ok(())
}

async fn setup_offer_subscriber<R: NewsRepository + 'static>(
    amqp_url: &str,
    service: Arc<NewsService<R>>,
) -> Result<(), lapin::Error> {
    let channel = setup_channel(amqp_url).await?;

    channel
        .queue_declare(
            "mi8.offer.created",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            "mi8.offer.created",
            "polymove.events",
            "offer.created",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            "mi8.offer.created",
            "mi8-offer-consumer",
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
                                "MI8 received offer.created: {} in {} (domain: {})",
                                event.id,
                                event.city,
                                event.domain
                            );
                            if let Err(e) = service
                                .increment_city_offer_stats(&event.city, &event.domain)
                                .await
                            {
                                tracing::error!("Failed to update city offer stats: {}", e);
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to deserialize offer.created event: {}", e);
                        }
                    }
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(e) => {
                    tracing::error!("Offer consumer error: {}", e);
                }
            }
        }
    });

    Ok(())
}
