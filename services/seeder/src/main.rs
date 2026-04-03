use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};
use tokio::time::sleep;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CreateOfferRequest {
    title: String,
    link: String,
    city: String,
    domain: String,
    salary: f64,
    start_date: String,
    end_date: String,
}

#[derive(Debug, Deserialize)]
struct OfferResponse {
    id: String,
    title: String,
    city: String,
    domain: String,
}

async fn wait_for_health(client: &Client, service_name: &str, url: &str) -> Result<()> {
    for attempt in 1..=30 {
        match client.get(url).send().await {
            Ok(response) if response.status().is_success() => {
                info!("{service_name} is ready");
                return Ok(());
            }
            Ok(response) => {
                warn!(
                    "{service_name} healthcheck attempt {attempt}/30 returned {}",
                    response.status()
                );
            }
            Err(error) => {
                warn!("{service_name} healthcheck attempt {attempt}/30 failed: {error}");
            }
        }

        sleep(Duration::from_secs(2)).await;
    }

    anyhow::bail!("Timed out waiting for {service_name}");
}

async fn create_offer(
    client: &Client,
    erasmumu_base_url: &str,
    payload: CreateOfferRequest,
) -> Result<OfferResponse> {
    let response = client
        .post(format!("{erasmumu_base_url}/offer"))
        .json(&payload)
        .send()
        .await
        .with_context(|| format!("Failed to create offer {}", payload.title))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Offer creation failed for {}: {} {}",
            payload.title,
            status,
            body
        );
    }

    response.json().await.context("Invalid offer response")
}

fn demo_offers() -> Result<Vec<CreateOfferRequest>> {
    serde_json::from_str(include_str!("../data/offers.json"))
        .context("Failed to parse embedded offers dataset")
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let erasmumu_base_url =
        env::var("ERASMUMU_BASE_URL").unwrap_or_else(|_| "http://erasmumu:3001".to_string());

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .context("Failed to build HTTP client")?;

    wait_for_health(&client, "Erasmumu", &format!("{erasmumu_base_url}/health")).await?;

    let offers = demo_offers()?;

    info!("Seeding demo offers...");
    for offer in offers {
        let created = create_offer(&client, &erasmumu_base_url, offer).await?;
        info!(
            "Created offer: {} [{} - {}] -> {}",
            created.title, created.domain, created.city, created.id
        );
    }

    info!("Seeder completed successfully");
    Ok(())
}
