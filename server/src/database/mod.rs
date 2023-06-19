use postgres::{
    Client, 
    NoTls, 
    Error
};

use crate::config::Postgres;

pub mod dbo;

pub struct Database {
    client : Client
}

impl  Database {
    
    pub fn new(config : Postgres) -> Self {
        let connection_string = format!(
            "postgresql://{}:{}@{}/{}",
            config.user,
            config.password,
            config.hostname,
            config.database
        );

        Database {
            client : Client::connect(connection_string.as_str(), NoTls).unwrap()
        }
    }

    pub async fn init(&mut self) -> Result<(), Error> {
        self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS words (
                id              UUID PRIMARY KEY,
                word            VARCHAR NOT NULL
            )
        ")?;

        self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS words_xref_posts (
                id              UUID PRIMARY KEY,
                word_id         UUID NOT NULL,
                post_id         UUID NOT NULL
            )
        ")?;

        self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS posts (
                id              UUID PRIMARY KEY,
                title           VARCHAR NOT NULL,
                body            VARCHAR NULL,
                upvotes         INTEGER,
                last_updaate    DATE,
            )
        ")?;

        self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS comments (
                id              UUID PRIMARY KEY,
                post_id         UUID NOT NULL,
                body            VARCHAR NULL,
                upvotes         INTEGER,
                last_updaate    DATE,
            )
        ")?;

        Ok(())
    }
}
