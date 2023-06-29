use async_recursion::async_recursion;
use reqwest::Client;
use crate::{
    config,
    error::{
        Result,
        LogError, LemmySearchError
    },
    api::lemmy::{fetcher::Fetcher, models::post::PostData}, 
    database::{  
        dbo::{
            DBO, 
            site::SiteDBO,
            crawler::CrawlerDatabase
        }, 
        DatabasePool, schema::DatabaseSchema
    }
};

pub struct Crawler {
    pub instance : String,

    config : config::Crawler,
    pool : DatabasePool,
    fetcher : Fetcher,

    just_update_remote_ids : bool
}

impl Crawler {

    pub fn new(
        instance : String,
        config : config::Crawler,
        pool : DatabasePool,

        just_update_remote_ids : bool
    ) -> Result<Self> {
        let client = Client::builder()
            .connection_verbose(true)
            .build()?;

        Ok(Self {
            instance: instance.clone(),
            config,
            pool,
            fetcher: Fetcher::new(client, instance),
            just_update_remote_ids
        })
    }

    #[async_recursion]
    pub async fn crawl(
        &self
    ) -> Result<()> {
        let site_view = self.fetcher.fetch_site_data()
            .await
            .log_error(format!("\t...unable to fetch site data for instance '{}'.", self.instance).as_str(), self.config.log)
            ?.site_view;

        let site_actor_id = site_view.site.actor_id.clone();

        let site_dbo = SiteDBO::new(self.pool.clone());

        if !site_dbo.upsert(site_view.clone())
            .await
            .log_error(format!("\t...error during update {} during crawl.", site_dbo.get_object_name()).as_str(), self.config.log)? {
                println!("\t...failed to update {} during crawl.", site_dbo.get_object_name());
            }

        if self.just_update_remote_ids {
            self.fetch_remote_ids(&site_actor_id)
                .await?;
        } else {
            self.fetch_posts(&site_actor_id)
                .await?;

            let federated_instances = self.fetcher.fetch_instances()
                .await?
                .linked;
    
            for instance in federated_instances {
                if match instance.software {
                    Some(value) => value == "lemmy",
                    None => false
                } {
                    let cralwer = Crawler::new(
                        instance.domain, 
                        self.config.clone(), 
                        self.pool.clone(), 
                        true
                    )?;
                    cralwer.crawl()
                        .await?;
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

        let site_dbo = SiteDBO::new(self.pool.clone());

        let last_page = site_dbo.get_last_post_page(site_actor_id)
            .await?;

        let mut total_found = 0;
        let mut page = last_page;
        loop {
            let posts = self.fetcher.fetch_posts(page+1)
                .await
                .log_error(format!("\tfailed to fetch another page of {}...", PostData::get_table_name()).as_str(), self.config.log)?;

            if posts.is_empty() {
                break;
            }
            let count = posts.len();
            println!("\tfetched another {} {}...", count, PostData::get_table_name());

            let filtered_posts = posts.into_iter().filter(|post_data| {
                !post_data.post.deleted.unwrap_or(false) && !post_data.post.removed.unwrap_or(false)
            }).collect::<Vec<_>>();

            let filtered_count = filtered_posts.len();

            let pool = self.pool.clone();
            let site_actor_id_string = site_actor_id.to_string();

            tokio::task::spawn_blocking(move || -> Result<()> {
                let mut crawler_database = CrawlerDatabase::init(pool.clone())?;

                crawler_database.builk_update_post(&site_actor_id_string, &filtered_posts)
                    .log_error("\t...Bulk insert failed.", true)
            }).await.map_err(|_| {
                LemmySearchError::Unknown("Unknown".to_string())
            })??;

            total_found += filtered_count;

            println!("\tinserted {} {}...", total_found, PostData::get_table_name());

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

        let site_dbo = SiteDBO::new(self.pool.clone());
        // let lemmy_id_dbo = IdDBO::new(self.pool.clone());

        let last_page = site_dbo.get_last_post_page(site_actor_id)
            .await?;

        let mut crawler_database = CrawlerDatabase::init(self.pool.clone())?;

        let mut page = last_page;
        loop {
            let posts = self.fetcher.fetch_posts(page)
                .await
                .log_error("\tfailed to fetch another page of 'post ids'...", self.config.log)?;

            if posts.is_empty() {
                break;
            }

            println!("\tfetched another {} 'post ids'...", posts.len());

            crawler_database.bulk_update_lemmy_ids(site_actor_id, &posts)?;

            site_dbo.set_last_post_page(&site_actor_id, page)
                .await?;
            page += 1;
        }

        Ok(())
    }
}
