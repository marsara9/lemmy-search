use std::time::Duration;
use async_std::task::sleep;
use futures::Future;
use crate::{
    error::{
        LemmySearchError,
        LogError
    },
    api::lemmy::fetcher::Fetcher, 
    database::{
        Database,        
        dbo::{
            DBO, 
            site::SiteDBO, 
            community::CommunityDBO, 
            post::PostDBO, 
            comment::CommentDBO, 
            word::WordsDBO, 
            search::SearchDatabase
        }
    }, config
};

use super::analyizer::Analyizer;

pub struct Crawler {
    pub instance : String,

    config : config::Crawler,
    database : Database,
    fetcher : Fetcher,
    analyizer : Analyizer
}

impl Crawler {

    pub fn new(
        instance : String,
        config : config::Crawler,
        database : Database
    ) -> Self {
        Self {
            instance: instance.clone(),
            config,
            database,
            fetcher: Fetcher::new(instance),
            analyizer : Analyizer::new()
        }
    }

    pub async fn crawl(
        &self
    ) -> Result<(), LemmySearchError> {
        let site_view = self.fetcher.fetch_site_data()
            .await
            .log_error(format!("\t...unable to fetch site data for instance '{}'.", self.instance).as_str(), self.config.log)
            ?.site_view;

        let site_actor_id = site_view.site.actor_id.clone();

        let site_dbo = SiteDBO::new(self.database.pool.clone());
        let community_dbo = CommunityDBO::new(self.database.pool.clone());
        let post_dbo = PostDBO::new(self.database.pool.clone());
        let comment_dbo: CommentDBO = CommentDBO::new(self.database.pool.clone());

        if !site_dbo.upsert(site_view.clone())
            .await
            .log_error(format!("\t...error during update {} during crawl.", site_dbo.get_object_name()).as_str(), self.config.log)? {
                println!("\t...failed to update {} during crawl.", site_dbo.get_object_name());
            }

        self.fetch_paged_object(
            &site_actor_id,
            site_dbo.get_last_community_page(&site_actor_id)
                .await?, 
            site_view.counts.communities.unwrap_or(0) / Fetcher::DEFAULT_LIMIT + 1,
            community_dbo,
            |page| {
                self.fetcher.fetch_communities(page)
            },
            |_| async {
                Ok(())
            },
            |page| {
                site_dbo.set_last_community_page(&site_actor_id, page)
            }
        ).await?;

        self.fetch_paged_object(
            &site_actor_id,
            site_dbo.get_last_post_page(&site_actor_id)
                .await?, 
            site_view.counts.posts.unwrap_or(0) / Fetcher::DEFAULT_LIMIT + 1,
            post_dbo,
            |page| {
                self.fetcher.fetch_posts(page)
            },
            |post_data| async move {
                let words_dbo = WordsDBO::new(self.database.pool.clone());
                let search = SearchDatabase::new(self.database.pool.clone());
                let words = self.analyizer.get_distinct_words_in_post(&post_data.post);
                for word in words.clone() {
                    if !words_dbo.upsert(word)
                        .await
                        .log_error("\t...words insertion failed.", self.config.log)? {
                            println!("\t...failed to insert search content.")
                        }
                }
                search.upsert_post(words, post_data.post)
                    .await
                    .log_error("\t...search insertion failed.", self.config.log)
            },
            |page| {
                site_dbo.set_last_post_page(&site_actor_id, page)
            }
        ).await?;

        self.fetch_paged_object(
            &site_actor_id,
            site_dbo.get_last_comment_page(&site_actor_id)
                .await?, 
            site_view.counts.comments.unwrap_or(0) / Fetcher::DEFAULT_LIMIT + 1,
            comment_dbo,
            |page| {
                self.fetcher.fetch_comments(page)
            },
            |comment_data| async move {
                let words_dbo = WordsDBO::new(self.database.pool.clone());
                let search = SearchDatabase::new(self.database.pool.clone());
                let words = self.analyizer.get_distinct_words_in_comment(&comment_data.comment);
                for word in words.clone() {
                    if !words_dbo.upsert(word)
                        .await
                        .log_error("\t...words insertion failed.", self.config.log)? {
                            println!("\t...failed to insert search content.")
                        }
                }
                search.upsert_comment(words, comment_data.comment)
                    .await
                    .log_error("\t...search insertion failed.", self.config.log)
            },
            |page| {
                site_dbo.set_last_comment_page(&site_actor_id, page)
            }
        ).await?;

        println!("\t...done.");

        Ok(())
    }

    async fn fetch_paged_object<T, D, Fetcher, Insert, Updater>(
        &self,
        site_actor_id : &str,
        last_page : i32,
        total_pages : i32,
        object_dbo : D,
        fetcher : impl Fn(i32) -> Fetcher,
        do_on_insert : impl Fn(T) -> Insert,
        page_updater : impl Fn(i32) -> Updater
    ) -> Result<(), LemmySearchError> where 
        T : Clone + Default,
        Fetcher : Future<Output = Result<Vec<T>, LemmySearchError>>,
        Updater : Future<Output = Result<bool, LemmySearchError>>,
        Insert : Future<Output = Result<(), LemmySearchError>>,
        D : DBO<T> + Sized
    {
        println!("Fetching {} from '{}'...", object_dbo.get_object_name(), site_actor_id);

        let mut count = 0;        
        for page in last_page..total_pages {
            let objects = fetcher(page+1)
                .await
                .log_error(format!("\tfailed to fetch another page of {}...", object_dbo.get_object_name()).as_str(), self.config.log)?;
            println!("\tfetched another {} {}...", objects.len(), object_dbo.get_object_name());

            sleep( Duration::from_millis( 100 ) )
                .await;

            for object in objects {
                object_dbo.upsert(object.clone())
                    .await
                    .log_error(format!("\t...failed to insert after fetching {} objects", count).as_str(), self.config.log)?;

                do_on_insert(object.clone())
                    .await
                    .log_error("\t...building search queries failed.", self.config.log)?;

                count += 1;
            }

            page_updater(last_page + count)
                .await
                .log_error("\t...update last page failed.", self.config.log)?;

            println!("\tinserted {} {}...", count, object_dbo.get_object_name());
        }

        Ok(())
    }
}
