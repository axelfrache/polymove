use crate::mi8_proto::{CityScore, News};
use crate::ports::erasmumu_client::{ErasmumuClient, ErasmumuOffer};
use crate::ports::mi8_client::Mi8Client;
use crate::ports::student_repository::StudentRepository;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct EnrichedOffer {
    pub id: String,
    pub title: String,
    pub link: String,
    pub city: String,
    pub domain: String,
    pub salary: f64,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub scores: EnrichedScores,
    pub latest_news: Vec<EnrichedNews>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EnrichedScores {
    pub quality_of_life: i32,
    pub economy: i32,
    pub culture: i32,
    pub safety: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EnrichedNews {
    pub title: String,
    pub source: String,
    pub date: String,
    pub tags: Vec<String>,
}

impl EnrichedScores {
    pub fn default_scores() -> Self {
        Self {
            quality_of_life: 0,
            economy: 0,
            culture: 0,
            safety: 0,
        }
    }
}

pub struct OfferAggregationService<R: StudentRepository, E: ErasmumuClient, M: Mi8Client> {
    student_repository: Arc<R>,
    erasmumu_client: Arc<E>,
    mi8_client: Arc<M>,
}

impl<R, E, M> OfferAggregationService<R, E, M>
where
    R: StudentRepository + Send + Sync,
    E: ErasmumuClient + Send + Sync,
    M: Mi8Client + Send + Sync,
{
    pub fn new(
        student_repository: Arc<R>,
        erasmumu_client: Arc<E>,
        mi8_client: Arc<M>,
    ) -> Self {
        Self {
            student_repository,
            erasmumu_client,
            mi8_client,
        }
    }

    pub async fn get_enriched_offers(
        &self,
        city: Option<String>,
        domain: Option<String>,
        limit: usize,
    ) -> Result<Vec<EnrichedOffer>, anyhow::Error> {
        let offers = self.erasmumu_client.fetch_offers(city, domain).await?;
        
        let limited_offers: Vec<ErasmumuOffer> = offers.into_iter().take(limit).collect();
        
        let cities: HashSet<String> = limited_offers
            .iter()
            .map(|o| o.city.clone())
            .collect();

        let mut mi8_futures = Vec::new();
        for unique_city in cities {
            let mi8 = self.mi8_client.clone();
            let city_clone = unique_city.clone();
            let city_clone2 = unique_city.clone();
            
            let fut = async move {
                let score_result = mi8.get_city_score(city_clone).await;
                let news_result = mi8.get_latest_news_in_city(city_clone2, 3).await;
                (unique_city, score_result, news_result)
            };
            mi8_futures.push(fut);
        }

        let results = join_all(mi8_futures).await;
        
        let mut city_cache: HashMap<String, (EnrichedScores, Vec<EnrichedNews>)> = HashMap::new();
        
        for (unique_city, score_res, news_res) in results {
            let scores = match score_res {
                Ok(s) => EnrichedScores {
                    quality_of_life: s.quality_of_life,
                    economy: s.economy,
                    culture: s.culture,
                    safety: s.safety,
                },
                Err(e) => {
                    tracing::warn!("Failed to fetch city score for {}: {}", unique_city, e);
                    EnrichedScores::default_scores()
                }
            };
            
            let news = match news_res {
                Ok(n) => n.into_iter().map(|news_item| {
                    EnrichedNews {
                        title: news_item.name,
                        source: news_item.source,
                        date: news_item.date,
                        tags: news_item.tags,
                    }
                }).collect(),
                Err(e) => {
                    tracing::warn!("Failed to fetch latest news for {}: {}", unique_city, e);
                    vec![]
                }
            };
            
            city_cache.insert(unique_city, (scores, news));
        }

        let mut enriched_offers = Vec::new();
        for offer in limited_offers {
            let (scores, news) = city_cache
                .get(&offer.city)
                .cloned()
                .unwrap_or_else(|| (EnrichedScores::default_scores(), vec![]));
                
            enriched_offers.push(EnrichedOffer {
                id: offer.id,
                title: offer.title,
                link: offer.link,
                city: offer.city,
                domain: offer.domain,
                salary: offer.salary,
                start_date: offer.start_date,
                end_date: offer.end_date,
                scores,
                latest_news: news,
            });
        }

        Ok(enriched_offers)
    }

    pub async fn get_recommended_offers(
        &self,
        student_id: Uuid,
        limit: usize,
        sort_by: Option<String>,
    ) -> Result<(crate::domain::student::Student, Vec<EnrichedOffer>), anyhow::Error> {
        let student = self.student_repository
            .get(student_id)
            .await
            .map_err(|e| anyhow::anyhow!("Student not found: {}", e))?;

        let mut enriched_offers = self.get_enriched_offers(None, Some(student.domain.clone()), 100).await?;

        if let Some(sort) = sort_by {
            enriched_offers.sort_by(|a, b| {
                match sort.as_str() {
                    "safety" => b.scores.safety.cmp(&a.scores.safety),
                    "economy" => b.scores.economy.cmp(&a.scores.economy),
                    "quality_of_life" => b.scores.quality_of_life.cmp(&a.scores.quality_of_life),
                    "culture" => b.scores.culture.cmp(&a.scores.culture),
                    _ => std::cmp::Ordering::Equal,
                }
            });
        }

        let limited_offers = enriched_offers.into_iter().take(limit).collect();
        Ok((student, limited_offers))
    }
}
