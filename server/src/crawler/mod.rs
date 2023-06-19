pub mod analyizer;
pub mod crawler;

use crate::{
    config, 
    database::Database
};

use self::crawler::Crawler;
use std::time::Duration;
use clokwerk::{
    TimeUnits, 
    Job, 
    AsyncScheduler
};
use tokio::task::JoinHandle;

pub struct Runner {
    config : config::Crawler,
    handle : Option<JoinHandle<()>>,
    database : Database
}

impl Runner {
    pub fn new(
        config : config::Crawler,
        database : Database
    ) -> Self {
        Runner { 
            config,
            handle : None,
            database
        }
    }

    pub fn start(&mut self) {
        self.stop();

        let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

        let config = self.config.clone();
        let database = self.database.clone();

        scheduler.every(1.day())
            .at("07:00")
            .run(move || Self::run(config.to_owned(), database.to_owned()));

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

    async fn run(
        config : config::Crawler,
        database : Database
    ) {
        if config.enabled {
            Crawler::new(config.seed_instance, database)
                    .crawl()
                    .await;
        }
    }
}
