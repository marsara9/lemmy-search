use async_trait::async_trait;
use super::{
    DBO, 
    get_database_client
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
