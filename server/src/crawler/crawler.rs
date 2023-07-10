use std::time::Duration;

use async_recursion::async_recursion;
use reqwest::Client;
use crate::{
    error::{
        Result,
        LogError, 
        LemmySearchError
    },
    api::lemmy::fetcher::Fetcher, 
    database::{  
        dbo::{
            site::SiteDBO,
            crawler::CrawlerDatabase
        }, 
        schema::{
            DatabaseSchema, 
            site::Site,
            posts::Post
        }, Context
    }
};

pub struct Crawler {
    pub instance : String,

    context : Context,
    fetcher : Fetcher,

    just_update_remote_ids : bool
}

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

impl Crawler {

    pub fn new(
        instance : String,
        context : Context,

        just_update_remote_ids : bool
    ) -> Result<Self> {
        let client = Client::builder()
            .user_agent(APP_USER_AGENT)
            .connect_timeout(Duration::from_secs(1))
            .timeout(Duration::from_secs(10))
            .connection_verbose(true)
            .build()?;

        Ok(Self {
            instance: instance.clone(),
            context,
            fetcher: Fetcher::new(client, instance),
            just_update_remote_ids
        })
    }

    #[async_recursion]
    pub async fn crawl(
        &self
    ) -> Result<()> {

        if !self.fetcher.fetch_if_can_crawl(APP_USER_AGENT).await? {
            return Err(LemmySearchError::Generic("Crawling disabled by robots.txt"));
        }

        let site_view = self.fetcher.fetch_site_data()
            .await
            .log_error(format!("\t...unable to fetch site data for instance '{}'.", self.instance).as_str(), self.context.config.crawler.log)
            ?.site_view;

        let site_actor_id = site_view.site.actor_id.clone();

        let site_dbo = SiteDBO::new(self.context.pool.clone());

        if !site_dbo.upsert(site_view.clone())
            .await
            .log_error(format!("\t...error during update {} during crawl.", Site::get_table_name()).as_str(), self.context.config.crawler.log)? {
                println!("\t...failed to update {} during crawl.", Site::get_table_name());
            }

        if self.just_update_remote_ids {
            self.fetch_remote_ids(&site_actor_id)
                .await?;
        } else {
            self.fetch_posts(&site_actor_id)
                .await?;

            if !self.context.config.crawler.single_instance_only.unwrap_or(false) {
                let federated_instances = self.fetcher.fetch_instances()
                .await?
                .federated_instances
                .linked;
    
                for instance in federated_instances {
                    if !match instance.software {
                        Some(value) => value.to_lowercase() == "lemmy",
                        None => false
                    } {
                        // Federated instance isn't a lemmy instance; skip.
                        continue;
                    }

                    if instance.domain == self.instance {
                        // Federated instance is self; skip.
                        continue;
                    }

                    let crawler = Crawler::new(
                        instance.domain, 
                        self.context.clone(), 
                        true
                    );
                    
                    let _ = match crawler {
                        Ok(crawler) => crawler.crawl().await,
                        Err(_) => Ok(())
                    };
                }
            }
        }

        println!("\t...done.");

        Ok(())
    }

    async fn fetch_posts(
        &self,
        site_actor_id : &str
    ) -> Result<()> {

        let site_dbo = SiteDBO::new(self.context.pool.clone());

        let last_page = site_dbo.get_last_post_page(site_actor_id)
            .await?;

        let mut total_found = 0;
        let mut page = last_page;
        loop {
            let posts = match self.fetcher.fetch_posts(page+1)
                .await
                .log_error(format!("\tfailed to fetch another page of {}...", Post::get_table_name()).as_str(), self.context.config.crawler.log) {
                    Ok(value) => value,
                    Err(_) => {
                        // Fetch failed wait for 1 second and then try again.
                        tokio::time::sleep(Duration::from_secs(1))
                            .await;
                        continue
                    }
                };

            if posts.is_empty() {
                break;
            }
            let count = posts.len();
            println!("\tfetched another {} {}...", count, Post::get_table_name());

            let filtered_posts = posts.into_iter().filter(|post_data| {
                !post_data.post.deleted.unwrap_or(false) && !post_data.post.removed.unwrap_or(false)
            }).collect::<Vec<_>>();

            let filtered_count = filtered_posts.len();

            let site_actor_id_string = site_actor_id.to_string();

            let mut crawler_database = CrawlerDatabase::init(self.context.pool.clone()).await?;

            crawler_database.bulk_update_post(&site_actor_id_string, &filtered_posts)
                .await
                .log_error("\t...Bulk insert failed.", true)?;

            total_found += filtered_count;

            println!("\tinserted {} {}...", total_found, Post::get_table_name());

            site_dbo.set_last_post_page(&site_actor_id, page)
                .await?;
            page += 1;
        }

        // TODO: Need to fetch comments and index their content.

        Ok(())
    }

    async fn fetch_remote_ids(
        &self,
        site_actor_id : &str
    ) -> Result<()> {

        let site_dbo = SiteDBO::new(self.context.pool.clone());

        let last_page = site_dbo.get_last_post_page(site_actor_id)
            .await?;

        let mut crawler_database = CrawlerDatabase::init(self.context.pool.clone()).await?;

        let mut page = last_page;
        loop {
            let posts = self.fetcher.fetch_posts(page+1)
                .await
                .log_error("\tfailed to fetch another page of 'post ids'...", self.context.config.crawler.log)?;

            if posts.is_empty() {
                break;
            }

            println!("\tfetched another {} 'post ids'...", posts.len());

            crawler_database.bulk_update_lemmy_ids(site_actor_id, &posts).await?;

            site_dbo.set_last_post_page(&site_actor_id, page)
                .await?;
            page += 1;
        }

        Ok(())
    }
}
