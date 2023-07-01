pub mod analyzer;
pub mod crawler;

use self::crawler::Crawler;
use std::{time::Duration, path::Path};
use async_std::fs::remove_file;
use tokio::task::JoinHandle;
use crate::{
    config, 
    database::Database,
    error::{
        LogError,
        Result, LemmySearchError
    }
};
use clokwerk::{
    TimeUnits, 
    AsyncScheduler
};

pub struct Runner {
    config : config::Crawler,
    handle : Option<JoinHandle<()>>,
    database : Database
}

impl Runner {
    pub fn new(
        config : &config::Crawler,
        database : Database
    ) -> Self {
        Self { 
            config : config.to_owned(),
            handle : None,
            database
        }
    }

    pub fn start(&mut self) {
        self.stop();

        let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

        let config1 = self.config.clone();
        let database1 = self.database.clone();
        let config2 = self.config.clone();
        let database2 = self.database.clone();

        scheduler.every(6.hours())            
            .run(move || Self::run(config1.clone(), database1.clone()));

        scheduler.every(1.minutes())
            .run(move || Self::manual_check(config2.clone(), database2.clone()));

        self.handle = Some(tokio::spawn(async move {
            loop {
                scheduler.run_pending().await;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }));
    }

    pub fn stop(&mut self) {
        match &self.handle {
            Some(value) => value.abort(),
            None => {}
        }
        self.handle = None
    }

    async fn manual_check(
        config : config::Crawler,
        database : Database
    ) {
        let file = Path::new("/lemmy/config/crawl");
        if file.exists() {
            match remove_file("/lemmy/config/crawl")
                .await {
                    Ok(_) => {
                        Self::run(config.clone(), database.clone())
                            .await; 
                    },
                    Err(err) => {
                        let _ = Result::<()>::Err(LemmySearchError::from(err))
                            .log_error("Failed to delete manual crawl trigger.", config.log);
                    }
                }
        }
    }

    async fn run(
        config : config::Crawler,
        database : Database
    ) {
        if config.enabled {
            println!("Crawler is starting to index '{}'...", config.seed_instance);
            let _ = Crawler::new(config.seed_instance.clone(), config.clone(), database.pool, false)
                    .unwrap()
                    .crawl()
                    .await
                    .log_error(format!("The crawler for '{}' encountered an error.", config.seed_instance).as_str(), config.log);

            println!("Crawling complete.");
        } else {
            println!("Crawler is currently disabled; skipping...");
        }
    }
}
