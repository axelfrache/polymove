use crate::domain::subscriber::Subscriber;
use crate::ports::subscriber_repository::{SubscriberError, SubscriberRepository};

pub struct SubscriberService<R: SubscriberRepository> {
    repository: R,
}

impl<R: SubscriberRepository> SubscriberService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn register_student(
        &self,
        student_id: String,
        domain: String,
    ) -> Result<Subscriber, SubscriberError> {
        if let Ok(Some(existing)) = self.repository.get(&student_id).await {
            return Ok(existing);
        }
        let subscriber = Subscriber::new(student_id, domain);
        self.repository.create(&subscriber).await
    }

    pub async fn get_subscriber(
        &self,
        student_id: &str,
    ) -> Result<Subscriber, SubscriberError> {
        self.repository
            .get(student_id)
            .await?
            .ok_or(SubscriberError::NotFound)
    }

    pub async fn update_subscriber(
        &self,
        student_id: &str,
        channel: Option<String>,
        contact: Option<String>,
        enabled: Option<bool>,
    ) -> Result<Subscriber, SubscriberError> {
        let mut subscriber = self
            .repository
            .get(student_id)
            .await?
            .ok_or(SubscriberError::NotFound)?;

        if let Some(c) = channel {
            subscriber.channel = c;
        }
        if let Some(c) = contact {
            subscriber.contact = c;
        }
        if let Some(e) = enabled {
            subscriber.enabled = e;
        }

        self.repository.update(&subscriber).await
    }

    pub async fn unsubscribe(&self, student_id: &str) -> Result<(), SubscriberError> {
        self.repository.delete(student_id).await
    }

    pub async fn list_by_domain(
        &self,
        domain: &str,
    ) -> Result<Vec<Subscriber>, SubscriberError> {
        self.repository.list_by_domain(domain).await
    }

    pub async fn send_offer_alert(
        &self,
        offer_domain: &str,
        offer_title: &str,
        offer_city: &str,
    ) -> Result<(), SubscriberError> {
        let subscribers = self.repository.list_by_domain(offer_domain).await?;
        for sub in subscribers {
            if sub.enabled {
                tracing::info!(
                    "[MOCK ALERT] Sending to {} via {}: New offer '{}' in {}",
                    sub.contact,
                    sub.channel,
                    offer_title,
                    offer_city
                );
            }
        }
        Ok(())
    }
}
