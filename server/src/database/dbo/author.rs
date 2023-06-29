use std::hash::Hash;

use async_trait::async_trait;
use postgres::types::ToSql;
use crate::{
    api::lemmy::models::author::Author, 
    database::DatabasePool, 
    error::LemmySearchError
};
use super::{
    DBO, 
    get_database_client, schema::DatabaseSchema
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

impl DatabaseSchema for Author {

    fn get_table_name(

    ) -> String {
        "authors".to_string()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "ap_id".to_string(),
            "avatar".to_string(),
            "name".to_string(),
            "display_name".to_string()
        ]
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.actor_id,
            &self.avatar,
            &self.name,
            &self.display_name
        ]
    }
}

impl PartialEq for Author {
    fn eq(&self, other: &Self) -> bool {
        self.actor_id == other.actor_id
    }
}

impl Eq for Author {

}

impl Hash for Author {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.actor_id.hash(state);
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
}
