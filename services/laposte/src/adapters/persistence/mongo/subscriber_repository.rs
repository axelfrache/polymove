use crate::domain::subscriber::Subscriber;
use crate::ports::subscriber_repository::{SubscriberError, SubscriberRepository};
use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    Collection,
};

pub struct MongoSubscriberRepository {
    collection: Collection<Subscriber>,
}

impl MongoSubscriberRepository {
    pub fn new(collection: Collection<Subscriber>) -> Self {
        Self { collection }
    }
}

impl SubscriberRepository for MongoSubscriberRepository {
    async fn create(&self, subscriber: &Subscriber) -> Result<Subscriber, SubscriberError> {
        self.collection
            .insert_one(subscriber)
            .await
            .map_err(|e| SubscriberError::DatabaseError(e.to_string()))?;
        Ok(subscriber.clone())
    }

    async fn get(&self, student_id: &str) -> Result<Option<Subscriber>, SubscriberError> {
        self.collection
            .find_one(doc! { "student_id": student_id })
            .await
            .map_err(|e| SubscriberError::DatabaseError(e.to_string()))
    }

    async fn update(&self, subscriber: &Subscriber) -> Result<Subscriber, SubscriberError> {
        let result = self
            .collection
            .replace_one(doc! { "student_id": &subscriber.student_id }, subscriber)
            .await
            .map_err(|e| SubscriberError::DatabaseError(e.to_string()))?;

        if result.matched_count == 0 {
            return Err(SubscriberError::NotFound);
        }

        Ok(subscriber.clone())
    }

    async fn list_by_domain(&self, domain: &str) -> Result<Vec<Subscriber>, SubscriberError> {
        let cursor = self
            .collection
            .find(doc! { "domain": domain })
            .await
            .map_err(|e| SubscriberError::DatabaseError(e.to_string()))?;
        cursor
            .try_collect()
            .await
            .map_err(|e| SubscriberError::DatabaseError(e.to_string()))
    }

    async fn delete(&self, student_id: &str) -> Result<(), SubscriberError> {
        let result = self
            .collection
            .delete_one(doc! { "student_id": student_id })
            .await
            .map_err(|e| SubscriberError::DatabaseError(e.to_string()))?;

        if result.deleted_count == 0 {
            return Err(SubscriberError::NotFound);
        }

        Ok(())
    }
}
