use crate::mi8_proto::mi8_service_client::Mi8ServiceClient;
use crate::mi8_proto::{GetLatestNewsInCityRequest, GetLatestNewsRequest, News};
use tonic::transport::Channel;

#[derive(Clone)]
pub struct Mi8GrpcClient {
    client: Mi8ServiceClient<Channel>,
}

impl Mi8GrpcClient {
    pub async fn connect(addr: String) -> Result<Self, tonic::transport::Error> {
        let client = Mi8ServiceClient::connect(addr).await?;
        Ok(Self { client })
    }

    pub async fn get_latest_news(&self, limit: i32) -> Result<Vec<News>, tonic::Status> {
        let mut client = self.client.clone();
        let request = tonic::Request::new(GetLatestNewsRequest { limit });
        let response = client.get_latest_news(request).await?;
        Ok(response.into_inner().news)
    }

    pub async fn get_latest_news_in_city(
        &self,
        city: String,
        limit: i32,
    ) -> Result<Vec<News>, tonic::Status> {
        let mut client = self.client.clone();
        let request = tonic::Request::new(GetLatestNewsInCityRequest { city, limit });
        let response = client.get_latest_news_in_city(request).await?;
        Ok(response.into_inner().news)
    }
}
