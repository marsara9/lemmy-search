use chrono::Utc;
use uuid::Uuid;
use async_trait::async_trait;
use crate::{
    database::DatabasePool,
    api::lemmy::models::site::{
        SiteView, 
        Site
    }
};
use super::{
    DBO, 
    get_database_client
};

pub struct SiteDBO {
    pool : DatabasePool
}

impl SiteDBO {
    pub fn new(pool : DatabasePool) -> Self {
        return Self {
            pool
        }
    }
}

#[async_trait]
impl DBO<SiteView> for SiteDBO {

    async fn create_table_if_not_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS sites (
                    id              UUID PRIMARY KEY,
                    name            VARCHAR NULL,
                    actor_id        VARCHAR NOT NULL,
                    last_update     DATE
                )
            ", &[]
            )
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn create(
        &self, 
        object : &SiteView
    ) -> bool {
        let object = object.to_owned();
        match get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO sites (id, actor_id, laste_updated) 
                    VALUES ($1, $2, $3)",
                    &[
                        &Uuid::new_v4(),
                        &object.site.actor_id,
                        &Utc::now()
                    ]
            )
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn retrieve(
        &self,
        uuid : &Uuid,
    ) -> Option<SiteView> {
        let uuid = uuid.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one(
                "SELECT actor_id, name FROM sites WHERE id = $1",
                &[&uuid] 
            ) {
                Ok(row) => Some(SiteView {
                    site: Site {
                        actor_id: row.get(0),
                        name: row.get(1)
                    },
                    ..Default::default()
                }),
                Err(_) => None
            }
        }).await.unwrap_or(None)
    }

    async fn update(&self, uuid : &Uuid) -> bool {
        false
    }

    async fn delete(&self, uuid : &Uuid) -> bool {
        false
    }
}
