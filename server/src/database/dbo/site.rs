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

#[derive(Clone)]
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

    pub async fn set_last_community_page(
        &self,
        instance : &str,
        page : i64
    ) -> bool {
        let instance = instance.to_owned();
        match get_database_client(&self.pool, move |client| {
            client.execute("
                UPDATE sites
                    SET last_community_page = $2,
                    WHERE actor_id = $1
                ",&[
                    &instance, &page
                ]
            ).unwrap_or_default()
        }).await {
            Ok(value) => value == 1,
            Err(_) => false
        }
    }

    pub async fn set_last_post_page(
        &self,
        instance : &str,
        page : i64
    ) -> bool {
        let instance = instance.to_owned();
        match get_database_client(&self.pool, move |client| {
            client.execute("
                UPDATE sites
                    SET last_post_page = $2,
                    WHERE actor_id = $1
                ",&[
                    &instance, &page
                ]
            ).unwrap_or_default()
        }).await {
            Ok(value) => value == 1,
            Err(_) => false
        }
    }

    pub async fn set_last_comment_page(
        &self,
        instance : &str,
        page : i64
    ) -> bool {
        let instance = instance.to_owned();
        match get_database_client(&self.pool, move |client| {
            client.execute("
                UPDATE sites
                    SET last_comment_page = $2,
                    WHERE actor_id = $1
                ",&[
                    &instance, &page
                ]
            ).unwrap_or_default()
        }).await {
            Ok(value) => value == 1,
            Err(_) => false
        }
    }

    pub async fn get_last_community_page(
        &self,
        instance : &str
    ) -> i64 {
        let instance = instance.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT last_community_page 
                    FROM sites
                    WHERE actor_id = $1
                ",
                &[&instance]
            ) {
                Ok(row) => row.get(0),
                Err(_) => 0
            }
        }).await.unwrap_or(0)
    }

    pub async fn get_last_post_page(
        &self,
        instance : &str
    ) -> i64 {
        let instance = instance.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT last_post_page 
                    FROM sites
                    WHERE actor_id = $1
                ",
                &[&instance]
            ) {
                Ok(row) => row.get(0),
                Err(_) => 0
            }
        }).await.unwrap_or(0)
    }

    pub async fn get_last_comment_page(
        &self,
        instance : &str
    ) -> i64 {
        let instance = instance.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT last_comment_page 
                    FROM sites
                    WHERE actor_id = $1
                ",
                &[&instance]
            ) {
                Ok(row) => row.get(0),
                Err(_) => 0
            }
        }).await.unwrap_or(0)
    }
}

#[async_trait]
impl DBO<SiteView> for SiteDBO {

    fn get_object_name(&self) -> &str {
        "SiteView"
    }

    async fn create_table_if_not_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS sites (
                    id                  UUID PRIMARY KEY,
                    name                VARCHAR NULL,
                    actor_id            VARCHAR NOT NULL UNIQUE,
                    last_community_page INTEGER DEFAULT 0,
                    last_post_page      INTEGER DEFAULT 0,
                    last_comment_page   INTEGER DEFAULT 0,
                    last_update         DATE
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
            client.execute("DROP TABLE IF EXISTS sites", &[])
                .unwrap_or_default()
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn retrieve(
        &self,
        ap_id : &str
    ) -> Option<SiteView> {
        let ap_id = ap_id.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT name 
                    FROM sites
                    WHERE actor_id = $1
                ",
                &[&ap_id] 
            ) {
                Ok(row) => Some(SiteView {
                    site: Site {
                        actor_id : ap_id,
                        name: row.get("name")
                    },
                    ..Default::default()
                }),
                Err(_) => None
            }
        }).await.unwrap_or(None)
    }

    async fn upsert(
        &self,
        object : SiteView
    ) -> bool {
        match get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO sites (id, name, actor_id, last_update) 
                    VALUES ($1, $2, $3, $4)
                ON CONFLICT (actor_id)
                DO UPDATE SET name = $2, last_update = $4
                ",
                    &[
                        &Uuid::new_v4(),
                        &object.site.name,
                        &object.site.actor_id,
                        &Utc::now()
                    ]
            ).unwrap_or_default()
        }).await {
            Ok(value) => value == 1,
            Err(_) => false
        }
    }
}
