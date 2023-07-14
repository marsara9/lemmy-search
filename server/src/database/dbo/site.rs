use crate::{
    error::Result,
    database::DatabasePool
};

use super::get_database_client;

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

    pub async fn set_last_post_page(
        &self,
        ap_id : &str,
        page : i32
    ) -> Result<bool> {

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
        }).await
    }

    #[allow(unused)]
    pub async fn set_last_comment_page(
        &self,
        ap_id : &str,
        page : i32
    ) -> Result<bool> {

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
        }).await
    }

    pub async fn get_last_post_page(
        &self,
        ap_id : &str
    ) -> Result<i32> {

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
        }).await
    }

    #[allow(unused)]
    pub async fn get_last_comment_page(
        &self,
        ap_id : &str
    ) -> Result<i32> {

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
        }).await
    }
}
