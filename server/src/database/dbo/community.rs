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
    api::lemmy::models::community::Community
};

#[derive(Clone)]
pub struct CommunityDBO {
    pool : DatabasePool
}

impl CommunityDBO {
    pub fn new(pool : DatabasePool) -> Self {
        return Self {
            pool
        }
    }
}

impl DatabaseSchema for Community {

    fn get_table_name(

    ) -> String {
        "communities".to_string()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "ap_id".to_string(),
            "icon".to_string(),
            "name".to_string(),
            "title".to_string()
        ]
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.actor_id,
            &self.icon,
            &self.name,
            &self.title
        ]
    }
}

impl PartialEq for Community {
    fn eq(&self, other: &Self) -> bool {
        self.actor_id == other.actor_id
    }
}

impl Eq for Community {

}

impl Hash for Community {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.actor_id.hash(state);
    }
}

#[async_trait]
impl DBO<Community> for CommunityDBO {

    fn get_object_name(&self) -> &str {
        "CommunityData"
    }

    async fn create_table_if_not_exists(
        &self
    ) -> Result<(), LemmySearchError> {
        get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS communities (
                    ap_id             VARCHAR PRIMARY KEY,
                    icon              VARCHAR NULL,
                    name              VARCHAR NOT NULL,
                    title             VARCHAR NULL,
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
            client.execute("DROP TABLE IF EXISTS communities", &[])
                .map(|_| {
                    ()
                })
        })
    }
}
