use chrono::Utc;
use uuid::Uuid;
use async_trait::async_trait;
use super::{
    DBO, 
    get_database_client    
};
use crate::{
    error::LemmySearchError,
    database::DatabasePool,
    api::lemmy::models::site::{
        SiteView, 
        Site
    }
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

    pub async fn upsert(
        &self,
        object : SiteView
    ) -> Result<bool, LemmySearchError> {

        get_database_client(&self.pool, move |client| {

            client.execute("
                INSERT INTO sites (\"id\", \"name\", \"actor_id\", \"last_update\") 
                    VALUES ($1, $2, $3, $4)
                ON CONFLICT (actor_id)
                DO UPDATE SET \"name\" = $2, \"last_update\" = $4
                ",
                    &[
                        &Uuid::new_v4(),
                        &object.site.name,
                        &object.site.actor_id,
                        &Utc::now()
                    ]
            ).map(|count| {
                count == 1
            })
        })
    }

    pub async fn retrieve_all(
        &self
    ) -> Result<Vec<SiteView>, LemmySearchError> {

        get_database_client(&self.pool, move |client| {

            client.query("
                SELECT actor_id, name 
                    FROM sites
                ",
                &[] 
            ).map(|rows| {
                rows.iter().map(|row| {
                    SiteView {
                        site: Site { 
                            actor_id: row.get(0),
                            name: row.get(1)
                        },
                        ..Default::default()
                    }
                }).collect()
            })
        })
    }

    pub async fn set_last_post_page(
        &self,
        ap_id : &str,
        page : i32
    ) -> Result<bool, LemmySearchError> {

        let ap_id = ap_id.to_owned();
        
        get_database_client(&self.pool, move |client| {

            client.execute("
                UPDATE sites
                    SET last_post_page = $2
                    WHERE actor_id = $1
                ",&[
                    &ap_id, &page
                ]
            ).map(|count| {
                count == 1
            })
        })
    }

    #[allow(unused)]
    pub async fn set_last_comment_page(
        &self,
        ap_id : &str,
        page : i32
    ) -> Result<bool, LemmySearchError> {

        let ap_id = ap_id.to_owned();
        
        get_database_client(&self.pool, move |client| {

            client.execute("
                UPDATE sites
                    SET last_comment_page = $2
                    WHERE actor_id = $1
                ",&[
                    &ap_id, &page
                ]
            ).map(|count| {
                count == 1
            })
        })
    }

    pub async fn get_last_post_page(
        &self,
        ap_id : &str
    ) -> Result<i32, LemmySearchError> {

        let ap_id = ap_id.to_owned();

        get_database_client(&self.pool, move |client| {

            client.query_one("
                SELECT last_post_page 
                    FROM sites
                    WHERE actor_id = $1
                ",
                &[&ap_id]
            ).map(|row| {
                row.get("last_post_page")
            })
        })
    }

    #[allow(unused)]
    pub async fn get_last_comment_page(
        &self,
        ap_id : &str
    ) -> Result<i32, LemmySearchError> {

        let ap_id = ap_id.to_owned();
        
        get_database_client(&self.pool, move |client| {

            client.query_one("
                SELECT last_comment_page 
                    FROM sites
                    WHERE actor_id = $1
                ",
                &[&ap_id]
            ).map(|row| {
                row.get("last_comment_page")
            })
        })
    }
}

#[async_trait]
impl DBO<SiteView> for SiteDBO {

    fn get_object_name(&self) -> &str {
        "SiteView"
    }

    async fn create_table_if_not_exists(
        &self
    ) -> Result<(), LemmySearchError> {

        get_database_client(&self.pool, move |client| {

            client.execute("
                CREATE TABLE IF NOT EXISTS sites (
                    id                  UUID PRIMARY KEY,
                    name                VARCHAR NULL,
                    actor_id            VARCHAR NOT NULL UNIQUE,
                    last_post_page      INTEGER DEFAULT 0,
                    last_comment_page   INTEGER DEFAULT 0,
                    last_update         TIMESTAMP WITH TIME ZONE NOT NULL
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

        get_database_client(&self.pool, move |client| {

            client.execute("DROP TABLE IF EXISTS sites", &[])
                .map(|_| {
                    ()
                })
        })
    }
}
