use dotenvy::dotenv;
use polytech::{
    adapters::{
        grpc::mi8_client::Mi8GrpcClient, http, persistence::postgres::PostgresStudentRepository,
    },
    application::student_service::StudentService,
};
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "polytech=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let host = env::var("POLYTECH_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("POLYTECH_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr_str = format!("{}:{}", host, port);

    let mi8_addr = env::var("MI8_GRPC_ADDR").unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());
    let erasmumu_base_url = env::var("ERASMUMU_BASE_URL").unwrap_or_else(|_| "http://erasmumu:8082".to_string());
    let upstream_timeout_ms: u64 = env::var("UPSTREAM_TIMEOUT_MS")
        .unwrap_or_else(|_| "800".to_string())
        .parse()
        .unwrap_or(800);

    tracing::info!("Connecting to MI8 at {}...", mi8_addr);
    let mi8_client = Arc::new(Mi8GrpcClient::connect(mi8_addr, upstream_timeout_ms).await?);

    tracing::info!("Initializing Erasmumu client at {}...", erasmumu_base_url);
    let erasmumu_client = Arc::new(polytech::adapters::http::erasmumu_client::ErasmumuReqwestClient::new(
        erasmumu_base_url,
        upstream_timeout_ms,
    ));

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let repository = PostgresStudentRepository::new(pool);
    let service = Arc::new(StudentService::new(repository.clone()));
    
    let offer_aggregation_service = Arc::new(polytech::application::offer_aggregation_service::OfferAggregationService::new(
        Arc::new(repository),
        erasmumu_client.clone(),
        mi8_client.clone(),
    ));

    let app = http::router(service, mi8_client, offer_aggregation_service).await;

    let addr: SocketAddr = addr_str.parse()?;
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
