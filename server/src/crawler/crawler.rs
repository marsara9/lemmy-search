use std::time::Duration;
use async_std::task::sleep;
use futures::Future;
use async_recursion::async_recursion;
use crate::{
    config,
    error::{
        LemmySearchError,
        LogError
    },
    api::lemmy::{
        fetcher::Fetcher, 
        models::id::LemmyId
    }, 
    database::{
        Database,        
        dbo::{
            DBO, 
            site::SiteDBO, 
            community::CommunityDBO, 
            post::PostDBO, 
            comment::CommentDBO, 
            word::WordsDBO, 
            search::SearchDatabase, 
            author::AuthorDBO, 
            id::IdDBO
        }
    }
};

use super::analyizer::Analyizer;

pub struct Crawler {
    pub instance : String,

    config : config::Crawler,
    database : Database,
    fetcher : Fetcher,
    analyizer : Analyizer,

    just_update_remote_ids : bool
}

impl Crawler {

    pub fn new(
        instance : String,
        config : config::Crawler,
        database : Database,

        just_update_remote_ids : bool
    ) -> Self {
        Self {
            instance: instance.clone(),
            config,
            database,
            fetcher: Fetcher::new(instance),
            analyizer : Analyizer::new(),
            just_update_remote_ids
        }
    }

    #[async_recursion]
    pub async fn crawl(
        &self
    ) -> Result<(), LemmySearchError> {
        let site_view = self.fetcher.fetch_site_data()
            .await
            .log_error(format!("\t...unable to fetch site data for instance '{}'.", self.instance).as_str(), self.config.log)
            ?.site_view;

        let site_actor_id = site_view.site.actor_id.clone();

        let site_dbo = SiteDBO::new(self.database.pool.clone());

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

            let domains = self.fetcher.fetch_instances()
                .await?
                .linked
                .into_iter()
                .map(|instance| instance.domain);
    
            for domain in domains {
                let cralwer = Crawler::new(
                    domain, 
                    self.config.clone(), 
                    self.database.clone(), 
                    true
                );
                cralwer.crawl()
                    .await?;
            }
        }

        println!("\t...done.");

        Ok(())
    }

    async fn fetch_posts(
        &self,
        site_actor_id : &str
    ) -> Result<(), LemmySearchError> {

        let words_dbo = WordsDBO::new(self.database.pool.clone());
        let search = SearchDatabase::new(self.database.pool.clone());
        let lemmy_id_dbo = IdDBO::new(self.database.pool.clone());

        let site_dbo = SiteDBO::new(self.database.pool.clone());
        let post_dbo = PostDBO::new(self.database.pool.clone());
        let author_dbo = AuthorDBO::new(self.database.pool.clone());
        let community_dbo = CommunityDBO::new(self.database.pool.clone());

        let last_page = site_dbo.get_last_post_page(site_actor_id)
            .await?;

        let mut page = last_page;
        loop {
            let posts = self.fetcher.fetch_posts(page)
                .await
                .log_error(format!("\tfailed to fetch another page of {}...", post_dbo.get_object_name()).as_str(), self.config.log)?;

            if posts.is_empty() {
                break;
            }
            let count = posts.len();
            println!("\tfetched another {} {}...", count, post_dbo.get_object_name());

            for post_data in posts {

                let clone_post = post_data.post.clone();

                post_dbo.upsert(post_data.clone())
                    .await
                    .log_error(format!("\t...failed to insert after fetching {} objects", count).as_str(), self.config.log)?;

                author_dbo.upsert(post_data.creator)
                    .await
                    .log_error("\t...failed to add author data", self.config.log)?;

                community_dbo.upsert(post_data.community)
                    .await
                    .log_error("\t...failed to add community data", self.config.log)?;

                let lemmy_id = LemmyId {
                    post_remote_id: post_data.post.id,
                    post_actor_id: post_data.post.ap_id,
                    instance_actor_id: site_actor_id.to_owned()
                };

                lemmy_id_dbo.upsert(lemmy_id)
                    .await
                    .log_error("\t...failed to insert remote ids.", self.config.log)?;

                let words = self.analyizer.get_distinct_words_in_post(&clone_post);
                for word in words.clone() {
                    if !words_dbo.upsert(word)
                        .await
                        .log_error("\t...an error occured during insertion of search words.", self.config.log)? {
                            println!("\t...failed to insert search words.")
                        }
                }
                search.upsert_post(words, &clone_post)
                    .await
                    .log_error("\t...building search queries failed.", self.config.log)?;
            }

            println!("\tinserted {} {}...", count, post_dbo.get_object_name());

            site_dbo.set_last_post_page(&site_actor_id, page)
                .await?;
            page += 1;
        }

        // self.fetch_paged_object(
        //     &site_actor_id,
        //     site_dbo.get_last_post_page(&site_actor_id)
        //         .await?, 
        //     post_dbo,
        //     |page| {
        //         self.fetcher.fetch_posts(page)
        //     },
        //     |post_data| async move {
        //         let author_dbo = AuthorDBO::new(self.database.pool.clone());
        //         let community_dbo = CommunityDBO::new(self.database.pool.clone());

        //         let words_dbo = WordsDBO::new(self.database.pool.clone());
        //         let search = SearchDatabase::new(self.database.pool.clone());
        //         let lemmy_id_dbo = IdDBO::new(self.database.pool.clone());

        //         author_dbo.upsert(post_data.creator)
        //             .await
        //             .log_error("\t...failed to add author data", self.config.log)?;

        //         community_dbo.upsert(post_data.community)
        //             .await
        //             .log_error("\t...failed to add community data", self.config.log)?;

        //         let lemmy_id = LemmyId {
        //             post_remote_id: post_data.post.id.clone(),
        //             post_actor_id: post_data.post.ap_id.clone(),
        //             instance_actor_id: site_actor_id.to_owned()
        //         };

        //         lemmy_id_dbo.upsert(lemmy_id)
        //             .await
        //             .log_error("\t...failed to insert remote ids.", self.config.log)?;

        //         let words = self.analyizer.get_distinct_words_in_post(&post_data.post);
        //         for word in words.clone() {
        //             if !words_dbo.upsert(word)
        //                 .await
        //                 .log_error("\t...an error occured during insertion of search words.", self.config.log)? {
        //                     println!("\t...failed to insert search words.")
        //                 }
        //         }
        //         search.upsert_post(words, post_data.post)
        //             .await
        //     },
        //     |page| {
        //         site_dbo.set_last_post_page(&site_actor_id, page)
        //     }
        // ).await?;

        // self.fetch_paged_object(
        //     &site_actor_id,
        //     site_dbo.get_last_comment_page(&site_actor_id)
        //         .await?, 
        //     comment_dbo,
        //     |page| {
        //         self.fetcher.fetch_comments(page)
        //     },
        //     |comment_data| async move {
        //         let words_dbo = WordsDBO::new(self.database.pool.clone());
        //         let search = SearchDatabase::new(self.database.pool.clone());

        //         let words = self.analyizer.get_distinct_words_in_comment(&comment_data.comment);
        //         for word in words.clone() {
        //             if !words_dbo.upsert(word)
        //                 .await
        //                 .log_error("\t...an error occured during insertion of search words.", self.config.log)? {
        //                     println!("\t...failed to insert search words.")
        //                 }
        //         }
        //         search.upsert_comment(words, comment_data.comment)
        //             .await
        //     },
        //     |page| {
        //         site_dbo.set_last_comment_page(&site_actor_id, page)
        //     }
        // ).await?;

        Ok(())
    }

    async fn fetch_remote_ids(
        &self,
        site_actor_id : &str
    ) -> Result<(), LemmySearchError> {

        let site_dbo = SiteDBO::new(self.database.pool.clone());
        let lemmy_id_dbo = IdDBO::new(self.database.pool.clone());

        let last_page = site_dbo.get_last_post_page(site_actor_id)
            .await?;

        let mut page = last_page;
        loop {
            let posts = self.fetcher.fetch_posts(page)
                .await?;
                //.log_error(format!("\tfailed to fetch another page of {}...", object_dbo.get_object_name()).as_str(), self.config.log)?;

            if posts.is_empty() {
                break;
            }

            for post_data in posts {
                let lemmy_id = LemmyId {
                    post_remote_id: post_data.post.id,
                    post_actor_id: post_data.post.ap_id,
                    instance_actor_id: site_actor_id.to_owned()
                };

                lemmy_id_dbo.upsert(lemmy_id)
                    .await
                    .log_error("\t...failed to insert remote ids.", self.config.log)?;
            }

            site_dbo.set_last_post_page(&site_actor_id, page)
                .await?;
            page += 1;
        }

        Ok(())
    }

    /**
     * Begin fetching objects from the target instance starting with the provided `last_page`.  This 
     * method will keep fetching objects in chunks of 50 (DEFAULT_FETCH_LIMIT) until an empty response
     * is returned.
     */
    async fn fetch_paged_object<T, D, Fetcher, Insert, Updater>(
        &self,
        site_actor_id : &str,
        last_page : i32,
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

        // The total number of objects interacted with
        let mut count = 0;
        // The current page number to query
        let mut page = last_page;
        loop {
            let objects = fetcher(page+1)
                .await
                .log_error(format!("\tfailed to fetch another page of {}...", object_dbo.get_object_name()).as_str(), self.config.log)?;
            if objects.len() == 0 {
                break;
            }
            println!("\tfetched another {} {}...", objects.len(), object_dbo.get_object_name());

            for object in objects {
                object_dbo.upsert(object.clone())
                    .await
                    .log_error(format!("\t...failed to insert after fetching {} objects", count).as_str(), self.config.log)?;

                do_on_insert(object.clone())
                    .await
                    .log_error("\t...building search queries failed.", self.config.log)?;

                count += 1;
            }

            page_updater(page)
                .await
                .log_error("\t...update last page failed.", self.config.log)?;

            println!("\tinserted {} {}...", count, object_dbo.get_object_name());
            page += 1;
        }

        Ok(())
    }
}
