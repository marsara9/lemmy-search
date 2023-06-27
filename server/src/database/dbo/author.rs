use async_trait::async_trait;
use chrono::Utc;
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
                    display_name      VARCHAR NULL,
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
    ) ->  Result<(),LemmySearchError> {

        get_database_client(&self.pool, |client| {
            client.execute("DROP TABLE IF EXISTS authors", &[])
                .map(|_| {
                    ()
                })
        })
    }

    async fn upsert(
        &self, 
        object : Author
    ) ->  Result<bool,LemmySearchError> {

        get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO authors (\"ap_id\", \"avatar\", \"name\", \"display_name\", \"last_update\")
                    VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (ap_id)
                DO UPDATE SET \"avatar\" = $2, \"name\" = $3, \"display_name\" = $4, \"last_update\" = $5
                ", &[
                    &object.actor_id,
                    &object.avatar,
                    &object.name,
                    &object.display_name,
                    &Utc::now()
                ]
            ).map(|count| {
                count == 1
            })
        })
    }
}
