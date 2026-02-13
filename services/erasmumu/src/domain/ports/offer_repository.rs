use crate::domain::offer::Offer;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OfferError {
    #[error("Offer not found")]
    NotFound,
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub trait OfferRepository: Send + Sync {
    fn create(
        &self,
        offer: &Offer,
    ) -> impl std::future::Future<Output = Result<Offer, OfferError>> + Send;

    fn get_by_id(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Offer>, OfferError>> + Send;

    fn list_by_domain(
        &self,
        domain: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Offer>, OfferError>> + Send;

    fn list_by_city(
        &self,
        city: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Offer>, OfferError>> + Send;

    fn update(
        &self,
        offer: &Offer,
    ) -> impl std::future::Future<Output = Result<Offer, OfferError>> + Send;

    fn delete(&self, id: &str) -> impl std::future::Future<Output = Result<(), OfferError>> + Send;
}
