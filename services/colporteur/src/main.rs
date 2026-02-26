use chrono::Utc;
use mi8_proto::mi8_service_client::Mi8ServiceClient;
use mi8_proto::{CreateNewsRequest, News};
use rand::Rng;
use tonic::transport::Channel;
use uuid::Uuid;

pub mod mi8_proto {
    tonic::include_proto!("mi8");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = std::env::var("MI8_GRPC_ADDR").unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());
    tracing::info!("Connecting to MI8 at {}", addr);

    let mut client = Mi8ServiceClient::connect(addr).await?;

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

        tracing::info!("Sending news: {} ({}) to {}", news.name, tag, city);

        let request = tonic::Request::new(CreateNewsRequest {
            news: Some(news.clone()),
        });

        match client.create_news(request).await {
            Ok(response) => {
                let resp = response.into_inner();
                tracing::info!("Success: {} (ID: {})", resp.message, resp.news_id);
            }
            Err(e) => {
                tracing::error!("Failed to create news: {}", e);
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
