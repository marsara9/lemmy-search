pub mod analyizer;
pub mod crawler;

use crate::{config, database::DatabasePool};

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
}

impl Runner {
    pub fn new(
        config : config::Crawler,
        pool : DatabasePool
    ) -> Self {
        Runner { 
            config,
            handle : None
        }
    }

    pub fn start(&mut self) {
        self.stop();

        let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

        let instance = self.config.seed_instance.to_owned();        

        scheduler.every(1.day())
            .at("03:00")
            .run(move || Self::run(instance.to_owned()));

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
        instance : String
    ) {
        Crawler::new(instance.to_owned())
                    .crawl()
                    .await;
    }
}
