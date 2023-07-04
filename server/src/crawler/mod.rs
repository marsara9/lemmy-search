pub mod analyzer;
pub mod crawler;

use self::crawler::Crawler;
use std::{
    time::Duration, 
    path::Path
};
use async_std::fs::remove_file;
use tokio::task::JoinHandle;
use crate::{
    database::Context,
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
    context : Context,
    handle : Option<JoinHandle<()>>,
}

impl Runner {
    pub fn new(
        context : Context
    ) -> Self {
        Self { 
            context,
            handle : None
        }
    }

    pub fn start(&mut self) {
        self.stop();

        let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

        let context1 = self.context.clone();
        let context2 = self.context.clone();

        scheduler.every(6.hours())            
            .run(move || Self::run_regular(context1.clone()));

        scheduler.every(1.minutes())
            .run(move || Self::manual_check(context2.clone()));

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
        context : Context
    ) {
        let file = Path::new("/lemmy/config/crawl");
        if file.exists() {
            match remove_file("/lemmy/config/crawl")
                .await {
                    Ok(_) => {
                        Self::run(context)
                            .await; 
                    },
                    Err(err) => {
                        let _ = Result::<()>::Err(LemmySearchError::from(err))
                            .log_error("Failed to delete manual crawl trigger.", context.config.crawler.log);
                    }
                }
        }
    }

    async fn run_regular(
        context : Context
    ) {
        if context.config.crawler.enabled {
            Self::run(context)
                .await;
        } else {
            println!("Crawler is currently disabled; skipping...");
        }
    }

    async fn run(
        context : Context
    ) {
        println!("Crawler is starting to index '{}'...", context.config.crawler.seed_instance);
            let _ = Crawler::new(context.config.crawler.seed_instance.clone(), context.clone(), false)
                    .unwrap()
                    .crawl()
                    .await
                    .log_error(format!("The crawler for '{}' encountered an error.", context.config.crawler.seed_instance).as_str(), context.config.crawler.log);

            println!("Crawling complete.");
    }
}
