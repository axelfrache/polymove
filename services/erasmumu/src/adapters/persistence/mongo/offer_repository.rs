use crate::domain::offer::Offer;
use crate::domain::ports::offer_repository::{OfferError, OfferRepository};
use futures_util::TryStreamExt;
use mongodb::Collection;
use mongodb::bson::doc;

pub struct MongoOfferRepository {
    collection: Collection<Offer>,
}

impl MongoOfferRepository {
    pub fn new(collection: Collection<Offer>) -> Self {
        Self { collection }
    }
}

impl OfferRepository for MongoOfferRepository {
    async fn create(&self, offer: &Offer) -> Result<Offer, OfferError> {
        self.collection
            .insert_one(offer)
            .await
            .map_err(|e: mongodb::error::Error| OfferError::DatabaseError(e.to_string()))?;
        Ok(offer.clone())
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Offer>, OfferError> {
        self.collection
            .find_one(doc! { "id": id })
            .await
            .map_err(|e: mongodb::error::Error| OfferError::DatabaseError(e.to_string()))
    }

    async fn list_by_domain(&self, domain: &str) -> Result<Vec<Offer>, OfferError> {
        let cursor = self
            .collection
            .find(doc! { "domain": domain })
            .await
            .map_err(|e: mongodb::error::Error| OfferError::DatabaseError(e.to_string()))?;
        cursor
            .try_collect()
            .await
            .map_err(|e: mongodb::error::Error| OfferError::DatabaseError(e.to_string()))
    }

    async fn list_by_city(&self, city: &str) -> Result<Vec<Offer>, OfferError> {
        let cursor = self
            .collection
            .find(doc! { "city": city })
            .await
            .map_err(|e: mongodb::error::Error| OfferError::DatabaseError(e.to_string()))?;
        cursor
            .try_collect()
            .await
            .map_err(|e: mongodb::error::Error| OfferError::DatabaseError(e.to_string()))
    }

    async fn update(&self, offer: &Offer) -> Result<Offer, OfferError> {
        let result = self
            .collection
            .replace_one(doc! { "id": &offer.id }, offer)
            .await
            .map_err(|e: mongodb::error::Error| OfferError::DatabaseError(e.to_string()))?;

        if result.matched_count == 0 {
            return Err(OfferError::NotFound);
        }

        Ok(offer.clone())
    }

    async fn delete(&self, id: &str) -> Result<(), OfferError> {
        let result = self
            .collection
            .delete_one(doc! { "id": id })
            .await
            .map_err(|e: mongodb::error::Error| OfferError::DatabaseError(e.to_string()))?;

        if result.deleted_count == 0 {
            return Err(OfferError::NotFound);
        }

        Ok(())
    }
}
