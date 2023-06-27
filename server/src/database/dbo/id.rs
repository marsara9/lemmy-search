use async_trait::async_trait;
use super::{
    DBO, 
    get_database_client
};
use crate::{
    database::DatabasePool, 
    api::lemmy::models::id::LemmyId, error::LemmySearchError
};

pub struct IdDBO {
    pool : DatabasePool
}

impl IdDBO {
    pub fn new(pool : DatabasePool) -> Self {
        Self {
            pool
        }
    }
}

#[async_trait]
impl DBO<LemmyId> for IdDBO {

    fn get_object_name(&self) ->  &str {
        "LemmyId"
    }

    async fn create_table_if_not_exists(
        &self
    ) -> Result<(), LemmySearchError> {
        get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS lemmy_ids (
                    post_remote_id      INT8 NOT NULL,
                    post_actor_id       VARCHAR NOT NULL,
                    instance_actor_id   VARCHAR NOT NULL,
                    UNIQUE (post_actor_id, instance_actor_id)
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
            client.execute("DROP TABLE IF EXISTS lemmy_ids", &[])
                .map(|_| {
                    ()
                })
        })
    }

    async fn upsert(
        &self,
        object : LemmyId
    ) -> Result<bool, LemmySearchError> {
        get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO lemmy_ids (\"post_remote_id\", \"post_actor_id\", \"instance_actor_id\") 
                    VALUES ($1, $2, $3)
                ON CONFLICT (post_actor_id, instance_actor_id)
                DO UPDATE SET post_remote_id = $1
                ", &[
                    &object.post_remote_id,
                    &object.post_actor_id,
                    &object.instance_actor_id
                ]
            ).map(|_| {
                true
            })
        })
    }
}
