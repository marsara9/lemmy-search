use async_trait::async_trait;
use uuid::Uuid;
use crate::{
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
        "String"
    }

    async fn create_table_if_not_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS words (
                    id              UUID PRIMARY KEY,
                    word            VARCHAR NOT NULL UNIQUE
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
            client.execute("DROP TABLE IF EXISTS words", &[])
                .unwrap_or_default()                
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }
    
    #[allow(unused_variables)] // this function isn't used but is required by the DBO trait.
    async fn retrieve(
        &self, 
        ap_id : &str
    ) -> Option<String> {
        None
    }

    async fn upsert(
        &self,
        object : String
    ) -> bool {
        let object = object.to_owned();
        match get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO words (id, word)
                    VALUES ($1, $2)
                    ON CONFLICT (word)
                    DO NOTHING
                ",
                    &[
                        &Uuid::new_v4(),
                        &object.to_lowercase()
                    ]
            ).unwrap_or_default()
        }).await {
            Ok(value) => value == 1,
            Err(_) => false
        }
    }
}
