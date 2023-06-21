use crate::{api::{search::models::search::SearchPost, lemmy::models::{comment::Comment, post::Post}}, database::DatabasePool};

use super::get_database_client;


pub struct SearchDatabase {
    pub pool : DatabasePool
}

impl SearchDatabase {

    async fn create_table_if_not_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS xref (
                    word_id         UUID NOT NULL,
                    post_remote_id  INTEGER NOT NULL,
                    post_instance   VARCHAR NOT NULL
                )
            ", &[]
            )
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn drop_table_if_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("DROP TABLE IF EXISTS comments", &[])
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn upsert_post(
        &self,
        post : &Post
    ) -> bool {
        false
    }

    async fn upsert_comment(
        &self,
        comment : &Comment
    ) {

    }

    async fn search(
        &self,
        query : &str,
        instance : &Option<String>,
        community : &Option<String>,
        author : &Option<String>
    ) -> Option<Vec<SearchPost>> {        
        let query = query.to_owned();
        match get_database_client(&self.pool, move|client| {
            client.execute("
                SELECT * FROM words AS w
                    JOIN posts AS p ON p.remote_id = w.post_remote_id AND p.instance = 
                WHERE w.word IN $1
            ", 
            &[
                &query
            ])
        }).await {
            Ok(_) => None,
            Err(_) => None
        }
    }
}
