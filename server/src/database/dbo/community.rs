use chrono::Utc;
use async_trait::async_trait;
use crate::{
    database::DatabasePool,
    api::lemmy::models::{
        community::{
            Community, 
            CommunityData            
        }
    }
};
use super::{
    DBO, 
    get_database_client
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
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS communities (
                    ap_id             VARCHAR PRIMARY KEY,
                    name              VARCHAR NOT NULL,
                    title             VARCHAR NULL,
                    late_update       DATE NOT NULL
                )
            ", &[]
            ).unwrap_or_default()
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn drop_table_if_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("DROP TABLE IF EXISTS communities", &[])
                .unwrap_or_default()
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn retrieve(
        &self, 
        ap_id : &str
    ) -> Option<CommunityData> {
        let ap_id = ap_id.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT name, title FROM communities
                    WHERE m.ap_id = $1
                ",
                &[&ap_id] 
            ) {
                Ok(row) => Some(CommunityData { 
                    community : Community { 
                        actor_id: ap_id, 
                        name: row.get("name"), 
                        title: row.get("title") 
                    },
                    ..Default::default()
                }),
                Err(_) => None
            }
        }).await.unwrap_or(None)
    }
    
    async fn upsert(
        &self,
        object : CommunityData
    ) -> bool {
        match get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO comments (ap_id, name, title, last_updated) 
                    VALUES ($1, $2, $3, $4)
                ON CONFLICT (ap_id)
                DO UPDATE SET (name = $2, title = $3, last_updated = $4)
                ",
                    &[
                        &object.community.actor_id,
                        &object.community.name,
                        &object.community.title,                        
                        &Utc::now()
                    ]
            ).unwrap_or_default()
        }).await {
            Ok(value) => value == 1,
            Err(_) => false
        }
    }
}