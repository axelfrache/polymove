use crate::mi8_proto::mi8_service_client::Mi8ServiceClient;
use crate::mi8_proto::{GetLatestNewsInCityRequest, GetLatestNewsRequest, News};
use tonic::transport::Channel;
use std::time::Duration;
use crate::ports::mi8_client::Mi8Client;

#[derive(Clone)]
pub struct Mi8GrpcClient {
    client: Mi8ServiceClient<Channel>,
    timeout: Duration,
}

impl Mi8GrpcClient {
    pub async fn connect(addr: String, timeout_ms: u64) -> Result<Self, tonic::transport::Error> {
        let client = Mi8ServiceClient::connect(addr).await?;
        Ok(Self { 
            client,
            timeout: Duration::from_millis(timeout_ms),
        })
    }
}

impl Mi8Client for Mi8GrpcClient {
    async fn get_latest_news(&self, limit: i32) -> Result<Vec<News>, anyhow::Error> {
        let mut client = self.client.clone();
        let mut request = tonic::Request::new(GetLatestNewsRequest { limit });
        request.set_timeout(self.timeout);
        let response = client.get_latest_news(request).await?;
        Ok(response.into_inner().news)
    }

    async fn get_city_score(&self, city: String) -> Result<crate::mi8_proto::CityScore, anyhow::Error> {
        let mut client = self.client.clone();
        let mut request = tonic::Request::new(crate::mi8_proto::GetCityScoreRequest { city });
        request.set_timeout(self.timeout);
        let response = client.get_city_score(request).await?;
        
        match response.into_inner().score {
            Some(score) => Ok(score),
            None => Err(anyhow::anyhow!("No city score returned")),
        }
    }

    async fn get_latest_news_in_city(
        &self,
        city: String,
        limit: i32,
    ) -> Result<Vec<News>, anyhow::Error> {
        let mut client = self.client.clone();
        let mut request = tonic::Request::new(crate::mi8_proto::GetLatestNewsInCityRequest { city, limit });
        request.set_timeout(self.timeout);
        let response = client.get_latest_news_in_city(request).await?;
        Ok(response.into_inner().news)
    }
}
