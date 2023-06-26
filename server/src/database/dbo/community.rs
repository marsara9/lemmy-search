use chrono::Utc;
use async_trait::async_trait;
use super::{
    DBO, 
    get_database_client
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
    
    async fn upsert(
        &self,
        object : Community
    ) -> Result<bool, LemmySearchError> {
        get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO communities (\"ap_id\", \"icon\", \"name\", \"title\", \"last_update\") 
                    VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (ap_id)
                DO UPDATE SET \"icon\" = $2, \"name\" = $3, \"title\" = $4, \"last_update\" = $5
                ", &[
                    &object.actor_id,
                    &object.icon,
                    &object.name,
                    &object.title,                        
                    &Utc::now()
                ]
            ).map(|count| {
                count == 1
            })
        })
    }
}
