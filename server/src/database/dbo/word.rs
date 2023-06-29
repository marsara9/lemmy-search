use async_trait::async_trait;
use crate::{
    error::LemmySearchError,
    database::DatabasePool
};
use super::{
    DBO,     
    get_database_client
};

#[derive(Clone)]
pub struct WordsDBO {
    pool : DatabasePool
}

impl WordsDBO {
    pub fn new(pool : DatabasePool) -> Self {
        return Self {
            pool
        }
    }
}



#[async_trait]
impl DBO<String> for WordsDBO {

    fn get_object_name(&self) -> &str {
        "Words"
    }

    async fn create_table_if_not_exists(
        &self
    ) -> Result<(), LemmySearchError> {

        get_database_client(&self.pool, move |client| {

            client.execute("
                CREATE TABLE IF NOT EXISTS words (
                    id              UUID PRIMARY KEY,
                    word            VARCHAR NOT NULL UNIQUE
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

            client.execute("DROP TABLE IF EXISTS words", &[])
                .map(|_| {
                    ()
                })
        })
    }
}
