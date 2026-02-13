use crate::domain::offer::Offer;
use crate::domain::ports::offer_repository::{OfferError, OfferRepository};
use uuid::Uuid;

pub struct OfferService<R: OfferRepository> {
    repository: R,
}

impl<R: OfferRepository> OfferService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_offer(
        &self,
        title: String,
        link: String,
        city: String,
        domain: String,
        salary: f64,
        start_date: String,
        end_date: String,
    ) -> Result<Offer, OfferError> {
        let offer = Offer {
            id: Uuid::new_v4().to_string(),
            title,
            link,
            city,
            domain,
            salary,
            start_date,
            end_date,
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
        title: Option<String>,
        link: Option<String>,
        city: Option<String>,
        domain: Option<String>,
        salary: Option<f64>,
        start_date: Option<String>,
        end_date: Option<String>,
        available: Option<bool>,
    ) -> Result<Offer, OfferError> {
        let existing = self
            .repository
            .get_by_id(id)
            .await?
            .ok_or(OfferError::NotFound)?;

        let updated = Offer {
            id: existing.id,
            title: title.unwrap_or(existing.title),
            link: link.unwrap_or(existing.link),
            city: city.unwrap_or(existing.city),
            domain: domain.unwrap_or(existing.domain),
            salary: salary.unwrap_or(existing.salary),
            start_date: start_date.unwrap_or(existing.start_date),
            end_date: end_date.unwrap_or(existing.end_date),
            available: available.unwrap_or(existing.available),
        };

        self.repository.update(&updated).await
    }

    pub async fn delete_offer(&self, id: &str) -> Result<(), OfferError> {
        self.repository.delete(id).await
    }
}
