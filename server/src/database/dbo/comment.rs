use chrono::Utc;
use async_trait::async_trait;
use super::{
    DBO, 
    get_database_client, 
};
use crate::{
    error::LemmySearchError,
    database::DatabasePool,
    api::lemmy::models::comment::CommentData 
};

#[derive(Clone)]
pub struct CommentDBO {
    pool : DatabasePool
}

impl CommentDBO {
    pub fn new(pool : DatabasePool) -> Self {
        return Self {
            pool
        }
    }
}

#[async_trait]
impl DBO<CommentData> for CommentDBO {

    fn get_object_name(&self) -> &str {
        "CommentData"
    }

    async fn create_table_if_not_exists(
        &self
    ) -> Result<(), LemmySearchError> {

        get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS comments (
                    ap_id             VARCHAR PRIMARY KEY,
                    content           VARCHAR NULL,
                    score             INTEGER,
                    author_actor_id   VARCHAR NOT NULL,
                    post_ap_id        VARCHAR NOT NULL,
                    community_ap_id   VARCHAR NOT NULL,
                    last_update       TIMESTAMP WITH TIME ZONE NOT NULL
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
            client.execute("DROP TABLE IF EXISTS comments", &[])
                .map(|_| {
                    ()
                })
        })
    }

    async fn upsert(
        &self,
        object : CommentData
    ) ->  Result<bool, LemmySearchError> {

        get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO comments (\"ap_id\", \"content\", \"score\", \"author_actor_id\", \"post_ap_id\", \"community_ap_id\", \"last_update\")
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (ap_id)
                DO UPDATE SET \"content\" = $2, \"score\" = $3, \"author_actor_id\" = $4, \"post_ap_id\" = $5, \"community_ap_id\" = $6, \"last_update\" = $7
                ", &[
                    &object.comment.ap_id,
                    &object.comment.content,
                    &object.counts.score,
                    &object.creator.actor_id,
                    &object.post.ap_id,
                    &object.community.actor_id,
                    &Utc::now()
                ]
            ).map(|count| {
                count == 1
            })
        })
    }
}
