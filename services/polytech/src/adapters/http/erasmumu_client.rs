use crate::ports::erasmumu_client::{ErasmumuClient, ErasmumuOffer};
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct ErasmumuReqwestClient {
    client: Client,
    base_url: String,
}

impl ErasmumuReqwestClient {
    pub fn new(base_url: String, timeout_ms: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .build()
            .expect("Failed to build reqwest client");
        Self { client, base_url }
    }
}

impl ErasmumuClient for ErasmumuReqwestClient {
    async fn fetch_offers(
        &self,
        city: Option<String>,
        domain: Option<String>,
    ) -> Result<Vec<ErasmumuOffer>, anyhow::Error> {
        let mut url = format!("{}/offer", self.base_url);
        let mut params = Vec::new();
        
        if let Some(c) = city {
            params.push(format!("city={}", c));
        } else if let Some(d) = domain {
            params.push(format!("domain={}", d));
        }
        
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Erasmumu returned status {}", response.status()));
        }
        
        let offers = response.json::<Vec<ErasmumuOffer>>().await?;
        Ok(offers)
    }
}
