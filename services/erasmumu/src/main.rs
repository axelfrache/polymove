use erasmumu::adapters::amqp::publisher::AmqpPublisher;
use erasmumu::adapters::http;
use erasmumu::adapters::persistence::mongo::offer_repository::MongoOfferRepository;
use erasmumu::application::offer_service::OfferService;
use mongodb::Client;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mongodb_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let mongodb_db = std::env::var("MONGODB_DB").unwrap_or_else(|_| "erasmumu".to_string());

    tracing::info!("Connecting to MongoDB...");
    let client = Client::with_uri_str(&mongodb_uri).await?;
    let database = client.database(&mongodb_db);
    let collection = database.collection("offers");

    let repository = MongoOfferRepository::new(collection);
    let service = Arc::new(OfferService::new(repository));

    // Initialize AMQP publisher
    let amqp_url = std::env::var("AMQP_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@127.0.0.1:5672/%2f".to_string());

    let publisher = match AmqpPublisher::new(&amqp_url).await {
        Ok(p) => {
            tracing::info!("Connected to RabbitMQ");
            Some(Arc::new(p))
        }
        Err(e) => {
            tracing::warn!(
                "Failed to connect to RabbitMQ: {}. Continuing without messaging.",
                e
            );
            None
        }
    };

    let app = http::router(service, publisher);

    let host = std::env::var("ERASMUMU_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("ERASMUMU_PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
