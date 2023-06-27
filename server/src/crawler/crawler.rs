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
            word::WordsDBO, 
            search::SearchDatabase, 
            author::AuthorDBO, 
            id::IdDBO
        }
    }
};

use super::analyzer::Analyzer;

pub struct Crawler {
    pub instance : String,

    config : config::Crawler,
    database : Database,
    fetcher : Fetcher,
    analyzer : Analyzer,

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
            analyzer : Analyzer::new(),
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

            let federated_instances = self.fetcher.fetch_instances()
                .await?
                .linked
                .into_iter();
    
            for instance in federated_instances {
                if match instance.software {
                    Some(value) => value == "lemmy",
                    None => false
                } {
                    let cralwer = Crawler::new(
                        instance.domain, 
                        self.config.clone(), 
                        self.database.clone(), 
                        true
                    );
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

        let mut total_found = 0;
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

                if post_data.post.deleted.unwrap_or(false) || post_data.post.removed.unwrap_or(false) {
                    continue;
                }

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

                let words = self.analyzer.get_distinct_words_in_post(&clone_post);
                for word in words.clone() {
                    if !words_dbo.upsert(word)
                        .await
                        .log_error("\t...an error occurred during insertion of search words.", self.config.log)? {
                            println!("\t...failed to insert search words.")
                        }
                }
                search.upsert_post(words, &clone_post)
                    .await
                    .log_error("\t...building search queries failed.", self.config.log)?;

                total_found += 1;
            }

            println!("\tinserted {} {}...", total_found, post_dbo.get_object_name());

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
    ) -> Result<(), LemmySearchError> {

        let site_dbo = SiteDBO::new(self.database.pool.clone());
        let lemmy_id_dbo = IdDBO::new(self.database.pool.clone());

        let last_page = site_dbo.get_last_post_page(site_actor_id)
            .await?;

        let mut page = last_page;
        loop {
            let posts = self.fetcher.fetch_posts(page)
                .await
                .log_error("\tfailed to fetch another page of 'post ids'...", self.config.log)?;

            if posts.is_empty() {
                break;
            }

            println!("\tfetched another {} 'post ids'...", posts.len());

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
}
