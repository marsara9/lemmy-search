use postgres::{
    Client, 
    NoTls, 
    Error
};

pub mod dbo;

pub struct Database {
    pub location : String,

    client : Client
}

impl  Database {
    
    pub fn new(location : String) -> Self {
        let location_str = location.as_str();
        Database {
            location : location.to_owned(),
            client : Client::connect(location_str, NoTls).unwrap()
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
