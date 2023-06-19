pub mod dbo;

use crate::config::Postgres;
use postgres::{
    NoTls, 
    Config
};
use r2d2_postgres::{
    PostgresConnectionManager, 
    r2d2::Pool
};

pub type DatabasePool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(Clone)]
pub struct Database {
    pub pool : DatabasePool
}

impl Database {
    
    pub fn new(config : Postgres) -> Self {
        let db_config = Config::new()
            .user(&config.user)
            .password(config.password)
            .host(&config.hostname)
            .port(config.port)
            .dbname(&config.database)
            .to_owned();

        let manager = PostgresConnectionManager::new(
            db_config, NoTls            
        );
        let pool = Pool::new(manager)
            .unwrap();

        Database {
            pool
        }
    }

    pub async fn init_database(
        &self,
    ) -> Result<(), postgres::Error> {
        let mut client = self.pool.get().unwrap();

        client.batch_execute("
            CREATE TABLE IF NOT EXISTS words (
                id              UUID PRIMARY KEY,
                word            VARCHAR NOT NULL
            )
        ")?;

        client.batch_execute("
            CREATE TABLE IF NOT EXISTS words_xref_posts (
                id              UUID PRIMARY KEY,
                word_id         UUID NOT NULL,
                post_id         UUID NOT NULL
            )
        ")?;

        client.batch_execute("
            CREATE TABLE IF NOT EXISTS posts (
                id              UUID PRIMARY KEY,
                title           VARCHAR NOT NULL,
                body            VARCHAR NULL,
                upvotes         INTEGER,
                last_updaate    DATE,
            )
        ")?;

        client.batch_execute("
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
