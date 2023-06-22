use crate::{
    api::lemmy::fetcher::Fetcher, 
    database::{
        Database,        
        dbo::{
            DBO, 
            site::SiteDBO
        }
    }
};

pub struct Crawler {
    pub instance : String,
    
    database : Database,
    fetcher : Fetcher,
}

impl Crawler {

    pub fn new(
        instacne : String,
        database : Database
    ) -> Self {
        Self {
            instance: instacne.clone(),
            database,
            fetcher: Fetcher::new(instacne)
        }
    }

    pub async fn crawl(
        &self
    ) {
        let site_view = self.fetcher.fetch_site_data()
            .await
            .unwrap()
            .site_view;

        SiteDBO::new(self.database.pool.clone())
            .create(&site_view)
            .await;

        let number_of_comments = site_view
            .counts
            .comments
            .unwrap_or(1);

        // for page in 0..(number_of_comments / Fetcher::DEFAULT_LIMIT) {
        //     let comments = self.fetcher.fetch_comments(page)
        //         .await;

        //     for comment_data in comments {
        //         let words = self.analyizer.get_distinct_words_in_comment(
        //             &comment_data.comment
        //         );
        //         println!("Words: {:?}", words);

        //         CommentDBO::new(self.database.pool.clone())
        //             .create(&self.instance, &comment_data, )
        //             .await;
        //     }
        // }
    }
}
