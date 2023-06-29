use std::hash::Hash;
use async_trait::async_trait;
use postgres::types::ToSql;
use super::{
    DBO, 
    get_database_client, 
    schema::DatabaseSchema
};
use crate::{
    database::DatabasePool, 
    api::lemmy::models::id::LemmyId, 
    error::LemmySearchError
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

impl DatabaseSchema for LemmyId {

    fn get_table_name(

    ) -> String {
        "lemmy_ids".to_string()
    }

    fn get_keys(
    
    ) -> Vec<String> {
        vec![
            "post_actor_id".to_string(),
            "instance_actor_id".to_string() 
        ]    
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "post_remote_id".to_string(),
            "post_actor_id".to_string(),
            "instance_actor_id".to_string()
        ]
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.post_remote_id,
            &self.post_actor_id,
            &self.instance_actor_id
        ]
    }
}

impl PartialEq for LemmyId {
    fn eq(&self, other: &Self) -> bool {
        self.post_actor_id == other.post_actor_id && self.instance_actor_id == other.instance_actor_id
    }
}

impl Eq for LemmyId {

}

impl Hash for LemmyId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.post_actor_id.hash(state);
        self.instance_actor_id.hash(state);
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
}
