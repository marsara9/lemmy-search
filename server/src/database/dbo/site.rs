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

    pub async fn retrieve_all(
        &self
    ) -> Vec<SiteView> {
        get_database_client(&self.pool, move |client| {
            match client.query("
                SELECT actor_id, name 
                    FROM sites
                ",
                &[] 
            ) {
                Ok(rows) => {
                    rows.iter().map(|row| {
                        SiteView {
                            site: Site { 
                                actor_id: row.get(0),
                                name: row.get(1)
                            },
                            ..Default::default()
                        }
                    }).collect()
                },
                Err(_) => Vec::<SiteView>::new()
            }
        }).await.unwrap_or(Vec::<SiteView>::new())
    }
}

#[async_trait]
#[allow(unused_variables)]
impl DBO<SiteView> for SiteDBO {

    async fn create_table_if_not_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS sites (
                    id                UUID PRIMARY KEY,
                    instance          VARCHAR NOT NULL,
                    name              VARCHAR NULL,
                    actor_id          VARCHAR NOT NULL UNIQUE,
                    last_update       DATE
                )
            ", &[]
            )
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn drop_table_if_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("DROP TABLE IF EXISTS sites", &[])
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn create(
        &self, 
        instance : &str,
        object : &SiteView
    ) -> bool {
        let instance = instance.to_owned();  
        let object = object.to_owned();
        match get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO sites (id, instance, name, actor_id, laste_updated) 
                    VALUES ($1, $2, $3)",
                    &[
                        &Uuid::new_v4(),
                        &instance,
                        &object.site.name,
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
        remote_id : &i64,
        instance : &str
    ) -> Option<SiteView> {
        let remote_id = remote_id.to_owned();
        let instance = instance.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT actor_id, name 
                    FROM sites
                    WHERE instance = $1
                ",
                &[&instance] 
            ) {
                Ok(row) => Some(SiteView {
                    site: Site {
                        actor_id : row.get(0),
                        name: row.get(1)
                    },
                    ..Default::default()
                }),
                Err(_) => None
            }
        }).await.unwrap_or(None)
    }

    async fn update(
        &self, 
        remote_id : &i64,
        instance : &str
    ) -> bool {
        false
    }

    async fn delete(
        &self, 
        remote_id : &i64,
        instance : &str
    ) -> bool {
        false
    }
}
