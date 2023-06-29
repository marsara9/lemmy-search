use std::hash::Hash;

use async_trait::async_trait;
use postgres::types::ToSql;
use super::{
    DBO, 
    get_database_client, 
    schema::DatabaseSchema
};
use crate::{
    error::LemmySearchError,
    database::DatabasePool,
    api::lemmy::models::post::PostData
};

#[derive(Clone)]
pub struct PostDBO {
    pool : DatabasePool
}

impl PostDBO {
    pub fn new(pool : DatabasePool) -> Self {
        return Self {
            pool
        }
    }
}

impl DatabaseSchema for PostData {

    fn get_table_name(

    ) -> String {
        "posts".to_string()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "ap_id".to_string(),
            "url".to_string(),
            "name".to_string(),
            "body".to_string(),
            "score".to_string(),
            "author_actor_id".to_string(),
            "community_ap_id".to_string(),
        ]
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.post.ap_id,
            &self.post.url,
            &self.post.name,
            &self.post.body,
            &self.counts.score,
            &self.creator.actor_id,
            &self.community.actor_id
        ]
    }
}

impl PartialEq for PostData {
    fn eq(&self, other: &Self) -> bool {
        self.post.ap_id == other.post.ap_id
    }
}

impl Eq for PostData {

}

impl Hash for PostData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.post.ap_id.hash(state);
    }
}

#[async_trait]
impl DBO<PostData> for PostDBO {

    fn get_object_name(&self) -> &str {
        "PostData"
    }

    async fn create_table_if_not_exists(
        &self
    ) -> Result<(), LemmySearchError> {
        get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS posts (
                    ap_id             VARCHAR PRIMARY KEY,
                    url               VARCHAR NULL,
                    name              VARCHAR NOT NULL,
                    body              VARCHAR NULL,
                    score             INTEGER,
                    author_actor_id   VARCHAR NOT NULL,
                    community_ap_id   VARCHAR NOT NULL
                )
            ", &[]
            ).map(|_| {
                ()
            })
        })
    }

    async fn drop_table_if_exists(
        &self
    ) -> Result<(), LemmySearchError> {
        get_database_client(&self.pool, |client| {
            client.execute("DROP TABLE IF EXISTS posts", &[])
                .map(|_| {
                    ()
                })
            })
    }
}
