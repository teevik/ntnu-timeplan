use crate::fetch::semesters::fetch_semesters;
use crate::shared_types::SemestersWithCurrent;
use reqwest::Client;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug)]
pub struct SemestersCache {
    client: Client,
    last_time_fetched: RwLock<Instant>,
    fetched_semesters: RwLock<Arc<SemestersWithCurrent>>,
}

impl SemestersCache {
    pub async fn new(client: Client) -> color_eyre::Result<Self> {
        let fetched_semesters = fetch_semesters(&client).await?;
        let last_time_fetched = Instant::now();

        Ok(Self {
            client,
            last_time_fetched: RwLock::new(last_time_fetched),
            fetched_semesters: RwLock::new(Arc::new(fetched_semesters)),
        })
    }

    pub async fn get_or_fetch(&self) -> color_eyre::Result<Arc<SemestersWithCurrent>> {
        const CACHE_DURATION: Duration = Duration::from_secs(60 * 60 * 24 * 7); // 1 week

        let last_time_fetched = *self.last_time_fetched.read().await;
        let cache_out_of_date = Instant::now() > (last_time_fetched + CACHE_DURATION);

        if cache_out_of_date {
            info!("Fetching semesters");

            let fetched_seme = fetch_semesters(&self.client).await?;

            *self.last_time_fetched.write().await = Instant::now();
            *self.fetched_semesters.write().await = Arc::new(fetched_seme);
        };

        let cached_courses = self.fetched_semesters.read().await.clone();

        Ok(cached_courses)
    }
}
