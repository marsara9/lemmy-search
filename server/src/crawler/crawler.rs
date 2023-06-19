use std::borrow::BorrowMut;

use crate::{
    api::lemmy::fetcher::Fetcher, database::{Database, self}
};
use super::analyizer::Analyizer;

pub struct Crawler {
    pub instance : String,
    
    database : Database,
    fetcher : Fetcher,
    analyizer : Analyizer
}

impl Crawler {

    pub fn new(
        instacne : String,
        database : Database
    ) -> Self {
        Crawler {
            instance: instacne.clone(),
            database,
            fetcher: Fetcher::new(instacne),
            analyizer: Analyizer::new()
        }
    }

    pub async fn crawl(
        &self
    ) {
        let number_of_comments = self.fetcher.fetch_site_data()
            .await
            .site_view
            .counts
            .comments;

        for page in 0..(number_of_comments / Fetcher::DEFAULT_LIMIT) {
            let comments = self.fetcher.fetch_comments(page)
                .await;

            for comment_data in comments {
                let words = self.analyizer.get_distinct_words_in_comment(
                    &comment_data.comment
                );
                println!("Words: {:?}", words);
                let _ = self.database.insert_comment(&comment_data.comment, words)
                    .await;
            }
        }
    }
}
