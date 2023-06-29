use std::hash::Hash;

use async_trait::async_trait;
use postgres::types::ToSql;
use uuid::Uuid;
use crate::{
    error::LemmySearchError,
    database::DatabasePool
};
use super::{
    DBO,     
    get_database_client, 
    schema::DatabaseSchema
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

#[derive(Debug)]
pub struct Word {
    pub id : Uuid,
    pub word : String
}

impl Word {
    pub fn from(word : String) -> Self {
        Self {
            id : Uuid::new_v4(),
            word
        }
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}

impl Eq for Word {

}

impl Hash for Word {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {        
        self.word.hash(state);
    }
}

impl DatabaseSchema for Word {

    fn get_table_name(

    ) -> String {
        "words".to_string()
    }

    fn get_keys(
    
    ) -> Vec<String> {
        Self::get_column_names()    
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "id".to_string(),
            "word".to_string()
        ]
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.id,
            &self.word
        ]
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
