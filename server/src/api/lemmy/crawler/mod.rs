pub mod fetcher;

use std::{
    time::Duration, 
    collections::HashMap, 
    vec
};
use async_recursion::async_recursion;
use reqwest::Client;
use self::fetcher::Fetcher;
use crate::{
    error::{
        Result,
        LogError, 
        LemmySearchError
    },
    database::{  
        dbo::{
            site::SiteDBO,
            crawler::CrawlerDatabase
        }, 
        schema::{
            DatabaseSchema, 
            site::Site,
            posts::{
                Post, 
                Comment
            }
        }, 
        Context
    }
};

use super::models::{comment::CommentData, site::Instance};

pub struct LemmyCrawler {
    pub instance : String,

    context : Context,
    fetcher : Fetcher
}

pub static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

impl LemmyCrawler {

    pub fn new(
        instance : String,
        context : Context,
    ) -> Result<Self> {
        let client = Client::builder()
            .user_agent(APP_USER_AGENT)
            .connect_timeout(Duration::from_secs(1))
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self {
            instance: instance.clone(),
            context,
            fetcher: Fetcher::new(client, instance)
        })
    }

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

        let site = Site::from(site_view);

        if !Site::upsert(self.context.pool.clone(), &site)
            .await
            .log_error(format!("\t...error during update {} during crawl.", Site::get_table_name()).as_str(), self.context.config.crawler.log)? {
                println!("\t...failed to update {} during crawl.", Site::get_table_name());
            }

        self.fetch_posts(&site_actor_id)
            .await?;

        if !self.context.config.crawler.single_instance_only.unwrap_or(false) {

            let federated_instances = self.fetcher.fetch_instances()
                .await?
                .federated_instances
                .linked;

            futures::future::join_all(federated_instances.into_iter()
                .filter(|instance| {
                    instance.software.clone().unwrap_or("".to_owned()) == "lemmy" && instance.domain != self.instance
                }).map(|instance| {
                    self.spawn_crawler(instance)
                })).await;
        }

        println!("\t...done.");

        Ok(())
    }

    #[async_recursion]
    async fn spawn_crawler(
        &self,
        instance : Instance
    ) -> Result<()> {
        let instance = instance.clone();

        match LemmyCrawler::new(
            instance.domain, 
            self.context.clone()
        ) {
            Ok(crawler) => crawler.crawl().await,
            Err(_) => Ok(())
        }
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

            let filtered_posts = posts.into_iter()
                .filter(|post_data| {
                    !post_data.post.deleted && !post_data.post.removed
                }).map(|post_data| {
                    (post_data.post.id, Post::from(&post_data))
                }).collect::<HashMap<_, _>>();

            let raw_posts = filtered_posts.iter().map(|(_, post)| {
                post.to_owned()
            }).collect::<Vec<_>>();

            let all_comments = futures::future::join_all(filtered_posts.keys()
                .into_iter()
                .map(|remote_id| {
                    self.fetch_comments_for_post(remote_id)
                }).collect::<Vec<_>>())
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .filter(|comment| {
                        !comment.comment.deleted && !comment.comment.removed
                    }).collect::<Vec<_>>();

            let mut grouped_comments = HashMap::<String, Vec<Comment>>::new();

            for comment in all_comments {
                let group = grouped_comments.entry(comment.post.ap_id.clone())
                    .or_insert(vec![]);

                group.push(Comment::from(&comment));
            }

            let filtered_count = filtered_posts.len();

            let mut crawler_database = CrawlerDatabase::init(self.context.pool.clone()).await?;

            crawler_database.bulk_update_post(
                &raw_posts,
                &grouped_comments
            ).await
                .log_error("\t...Bulk insert failed.", true)?;

            total_found += filtered_count;

            println!("\tinserted {} {}...", total_found, Post::get_table_name());

            site_dbo.set_last_post_page(&site_actor_id, page)
                .await?;
            page += 1;
        }

        Ok(())
    }

    async fn fetch_comments_for_post(
        &self,
        remote_post_id : &i64
    ) -> Result<Vec<CommentData>> {

        let mut all_comments = Vec::new();

        let mut page = 0;
        loop {
            let mut comments = match self.fetcher.fetch_comments(
                remote_post_id.clone(),
                page + 1
            ).await
                .log_error(format!("\tfailed to fetch another page of Comments...").as_str(), self.context.config.crawler.log) {
                    Ok(value) => value,
                    Err(_) => {
                        // Fetch failed wait for 1 second and then try again.
                        tokio::time::sleep(Duration::from_secs(1))
                            .await;
                        continue
                    }
                };

            if comments.is_empty() {
                break;
            }

            all_comments.append(&mut comments);
            page += 1;
        }

        Ok(all_comments)
    }
}
