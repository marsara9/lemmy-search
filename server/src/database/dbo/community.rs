use chrono::Utc;
use async_trait::async_trait;
use super::{
    DBO, 
    get_database_client
};
use crate::{
    error::LemmySearchError,
    database::DatabasePool,
    api::lemmy::models::{
        community::{
            Community, 
            CommunityData            
        }
    }
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
impl DBO<CommunityData> for CommunityDBO {

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

    async fn retrieve(
        &self, 
        ap_id : &str
    ) -> Result<CommunityData, LemmySearchError> {

        let ap_id = ap_id.to_owned();

        get_database_client(&self.pool, move |client| {
            client.query_one("
                SELECT name, title FROM communities
                    WHERE m.ap_id = $1
                ",
                &[&ap_id] 
            ).map(|row| {
                CommunityData { 
                    community : Community { 
                        actor_id: ap_id.to_string(), 
                        name: row.get("name"), 
                        title: row.get("title") 
                    },
                    ..Default::default()
                }
            })
        })
    }
    
    async fn upsert(
        &self,
        object : CommunityData
    ) -> Result<bool, LemmySearchError> {
        get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO communities (\"ap_id\", \"name\", \"title\", \"last_update\") 
                    VALUES ($1, $2, $3, $4)
                ON CONFLICT (ap_id)
                DO UPDATE SET \"name\" = $2, \"title\" = $3, \"last_update\" = $4
                ",
                    &[
                        &object.community.actor_id,
                        &object.community.name,
                        &object.community.title,                        
                        &Utc::now()
                    ]
            ).map(|count| {
                count == 1
            })
        })
    }
}
