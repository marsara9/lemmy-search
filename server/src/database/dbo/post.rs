use chrono::Utc;
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
                    ap_id               VARCHAR PRIMARY KEY,
                    url                 VARCHAR NULL,
                    name                VARCHAR NOT NULL,
                    body                VARCHAR NULL,
                    score               INTEGER,
                    author_actor_id     VARCHAR NOT NULL,
                    author_avatar       VARCHAR NULL,
                    author_name         VARCHAR NOT NULL,
                    author_display_name VARCHAR NULL,
                    community_ap_id     VARCHAR NOT NULL,
                    community_icon      VARCHAR NULL,
                    community_name      VARCHAR NOT NULL,
                    community_title     VARCHAR NULL,
                    last_update         TIMESTAMP WITH TIME ZONE NOT NULL
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

    async fn upsert(
        &self,
        object : PostData
    ) -> Result<bool, LemmySearchError> {
        get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO posts (\"ap_id\", \"url\", \"name\", \"body\", \"score\", \"author_actor_id\", \"community_ap_id\", \"last_update\") 
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (ap_id)
                DO UPDATE SET \"url\" = $2, \"name\" = $3, \"body\" = $4, \"score\" = $5, \"author_actor_id\" = $6, \"community_ap_id\" = $7, \"last_update\" = $8
                ", &[
                    &object.post.ap_id,
                    &object.post.url,
                    &object.post.name,
                    &object.post.body,
                    &object.counts.score,
                    &object.creator.actor_id,
                    &object.community.actor_id,
                    &Utc::now()
                ]
            ).map(|count| {
                count == 1
            })
        })
    }
}
