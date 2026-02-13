use crate::domain::offer::Offer;
use crate::domain::ports::offer_repository::{OfferError, OfferRepository};
use uuid::Uuid;

pub struct CreateOfferParams {
    pub title: String,
    pub link: String,
    pub city: String,
    pub domain: String,
    pub salary: f64,
    pub start_date: String,
    pub end_date: String,
}

pub struct UpdateOfferParams {
    pub title: Option<String>,
    pub link: Option<String>,
    pub city: Option<String>,
    pub domain: Option<String>,
    pub salary: Option<f64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub available: Option<bool>,
}

pub struct OfferService<R: OfferRepository> {
    repository: R,
}

impl<R: OfferRepository> OfferService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_offer(&self, params: CreateOfferParams) -> Result<Offer, OfferError> {
        let offer = Offer {
            id: Uuid::new_v4().to_string(),
            title: params.title,
            link: params.link,
            city: params.city,
            domain: params.domain,
            salary: params.salary,
            start_date: params.start_date,
            end_date: params.end_date,
            available: true,
        };
        self.repository.create(&offer).await
    }

    pub async fn get_offer(&self, id: &str) -> Result<Offer, OfferError> {
        let offer = self.repository.get_by_id(id).await?;
        match offer {
            Some(o) if o.available => Ok(o),
            _ => Err(OfferError::NotFound),
        }
    }

    pub async fn list_offers_by_domain(&self, domain: &str) -> Result<Vec<Offer>, OfferError> {
        let offers = self.repository.list_by_domain(domain).await?;
        Ok(offers.into_iter().filter(|o| o.available).collect())
    }

    pub async fn list_offers_by_city(&self, city: &str) -> Result<Vec<Offer>, OfferError> {
        let offers = self.repository.list_by_city(city).await?;
        Ok(offers.into_iter().filter(|o| o.available).collect())
    }

    pub async fn update_offer(
        &self,
        id: &str,
        params: UpdateOfferParams,
    ) -> Result<Offer, OfferError> {
        let existing = self
            .repository
            .get_by_id(id)
            .await?
            .ok_or(OfferError::NotFound)?;

        let updated = Offer {
            id: existing.id,
            title: params.title.unwrap_or(existing.title),
            link: params.link.unwrap_or(existing.link),
            city: params.city.unwrap_or(existing.city),
            domain: params.domain.unwrap_or(existing.domain),
            salary: params.salary.unwrap_or(existing.salary),
            start_date: params.start_date.unwrap_or(existing.start_date),
            end_date: params.end_date.unwrap_or(existing.end_date),
            available: params.available.unwrap_or(existing.available),
        };

        self.repository.update(&updated).await
    }

    pub async fn delete_offer(&self, id: &str) -> Result<(), OfferError> {
        self.repository.delete(id).await
    }
}
