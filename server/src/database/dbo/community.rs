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
                    title             VARCHAR NULL
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
