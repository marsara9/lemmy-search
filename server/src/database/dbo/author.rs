use async_trait::async_trait;
use crate::{
    api::lemmy::models::author::Author, 
    database::DatabasePool, 
    error::LemmySearchError
};
use super::{
    DBO, 
    get_database_client
};

pub struct AuthorDBO {
    pool : DatabasePool
}

impl AuthorDBO {
    pub fn new(pool : DatabasePool) -> Self {
        Self {
            pool
        }
    }
}

#[async_trait]
impl DBO<Author> for AuthorDBO {

    fn get_object_name(&self) ->  &str {
        return "Author"
    }

    async fn create_table_if_not_exists(
        &self
    ) ->  Result<(),LemmySearchError> {

        get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS authors (
                    ap_id             VARCHAR PRIMARY KEY,
                    avatar            VARCHAR NULL,
                    name              VARCHAR NOT NULL,
                    display_name      VARCHAR NULL
                )
            ", &[]
            ).map(|_| {
                ()
            })
        })
    }

    async fn drop_table_if_exists(
        &self
    ) ->  Result<(),LemmySearchError> {

        get_database_client(&self.pool, |client| {
            client.execute("DROP TABLE IF EXISTS authors", &[])
                .map(|_| {
                    ()
                })
        })
    }
}
