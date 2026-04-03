use laposte::adapters::amqp::subscriber;
use laposte::adapters::http;
use laposte::adapters::persistence::mongo::subscriber_repository::MongoSubscriberRepository;
use laposte::application::subscriber_service::SubscriberService;
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
    let mongodb_db = std::env::var("MONGODB_DB").unwrap_or_else(|_| "laposte".to_string());

    tracing::info!("Connecting to MongoDB...");
    let client = Client::with_uri_str(&mongodb_uri).await?;
    let database = client.database(&mongodb_db);
    let collection = database.collection("subscribers");

    let repository = MongoSubscriberRepository::new(collection);
    let service = Arc::new(SubscriberService::new(repository));

    let amqp_url = std::env::var("AMQP_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@127.0.0.1:5672/%2f".to_string());

    tracing::info!("Connecting to RabbitMQ...");
    if let Err(e) = subscriber::setup_and_subscribe(&amqp_url, service.clone()).await {
        tracing::error!("Failed to setup AMQP subscriber: {}", e);
    } else {
        tracing::info!("AMQP subscribers started successfully");
    }

    let app = http::router(service);

    let host = std::env::var("LAPOSTE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("LAPOSTE_PORT").unwrap_or_else(|_| "3002".to_string());
    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("La Poste listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
