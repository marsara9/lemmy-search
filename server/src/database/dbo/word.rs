use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    database::DatabasePool
};
use super::{
    DBO, 
    get_database_client
};

pub struct WordDAO {
    pool : DatabasePool
}

impl WordDAO {
    pub fn new(pool : DatabasePool) -> Self {
        return Self {
            pool
        }
    }
}

#[async_trait]
#[allow(unused_variables)]
impl DBO<String> for WordDAO {

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
            )
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
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn create(
        &self, 
        instance : &str,
        object : &String
    ) -> bool {
        let instance = instance.to_owned();  
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
                        &object
                    ]
            )
        }).await {
            Ok(_) => true,
            Err(_) => false
        } 
    }

    async fn retrieve(
        &self, 
        remote_id : &i64,
        instance : &str
    ) -> Option<String> {
        let remote_id = remote_id.to_owned();
        let instance = instance.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT word
                    FROM words
                    WHERE remote_id = $1 AND instance = $2
                ",
                &[&remote_id, &instance] 
            ) {
                Ok(row) => row.get(0),
                Err(_) => None
            }
        }).await.unwrap_or(None)
    }

    async fn update(
        &self, 
        remote_id : &i64,
        instance : &str
    ) -> bool {
        false
    }

    async fn delete(
        &self, 
        remote_id : &i64,
        instance : &str
    ) -> bool {
        false
    }
}
