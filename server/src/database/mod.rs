pub mod dbo;
pub mod schema;

use std::thread;

use crate::{
    config::Postgres, 
    database::{
        schema::{
            site::Site,
            word::Word, 
            xref::Search
        }
    }, 
    error::{
        Result, 
        LemmySearchError, LogError
    }, 
    api::lemmy::models::{
        author::Author, 
        community::Community, 
        post::PostData, 
        id::LemmyId
    }
};
use postgres::{
    NoTls, 
    Config
};
use r2d2_postgres::{
    PostgresConnectionManager, 
    r2d2::{
        Pool, 
        PooledConnection
    }
};

use self::schema::DatabaseSchema;

pub type DatabasePool = Pool<PostgresConnectionManager<NoTls>>;
pub type DatabaseClient = PooledConnection<PostgresConnectionManager<NoTls>>;

#[derive(Clone)]
pub struct Database {
    pub config : Postgres,
    pub pool : DatabasePool
}

impl Database {

    pub async fn create(
        config : &Postgres
    ) -> std::result::Result<Self, r2d2_postgres::r2d2::Error> {
        Self::create_database_pool(config)
            .await
            .map(|pool| {
                Database {
                    config : config.clone(),
                    pool
                }
            })
    }

    async fn create_database_pool(
        config : &Postgres
    ) -> std::result::Result<DatabasePool, r2d2_postgres::r2d2::Error> {
        let db_config = Config::new()
            .user(&config.user)
            .password(&config.password)
            .host(&config.hostname)
            .port(config.port)
            .dbname(&config.database)
            .to_owned();

        let manager = PostgresConnectionManager::new(
            db_config, NoTls            
        );
        Pool::new(manager)
    }

    pub fn init_database(
        &self,
    ) -> Result<()> {
        println!("Creating database...");

        let drop_table = false;

        self.create_table_from_schema::<Site>(drop_table)?;
        self.create_table_from_schema::<Author>(drop_table)?;
        self.create_table_from_schema::<Community>(drop_table)?;
        self.create_table_from_schema::<PostData>(drop_table)?;
        self.create_table_from_schema::<LemmyId>(drop_table)?;
        self.create_table_from_schema::<Word>(drop_table)?;
        self.create_table_from_schema::<Search>(drop_table)?;

        Ok(())
    }

    fn create_table_from_schema<S : DatabaseSchema>(
        &self,
        drop : bool
    ) -> Result<()> {
        let table_name = S::get_table_name();
        let column_names = S::get_column_names();
        let column_types = S::get_column_types();
        let primary_keys = S::get_keys();

        let columns = column_names.into_iter().map(|name| {
            format!("{}\t{}", name, column_types[&name].to_sql_type_name())
        }).collect::<Vec<_>>()
            .join(",\n");

        let primary_key = if primary_keys.is_empty() {
            "".to_string()
        } else {
            format!(", PRIMARY KEY ({})", primary_keys.join(", "))
        };

        println!("\tCreating '{}' table...", table_name);

        let drop_table = format!("
            DROP TABLE IF EXISTS {}
        ", table_name);

        let create_table = format!("
            CREATE TABLE IF NOT EXISTS {} (
                {}
                {}
            )
        ", table_name, columns, primary_key);

        let pool = self.pool.clone();
        let log: bool = self.config.log;

        thread::spawn(move || -> Result<()> {
            let mut client = pool.get()?;

            if drop {
                client.execute(&drop_table, &[])?;
            }

            client.execute(&create_table, &[]).map(|_| {
                ()
            }).map_err(|err| {
                LemmySearchError::Database(err)
            }).log_error(format!("...table creation failed for table '{}'", S::get_table_name()).as_str(), log)
        });

        Ok(())
    }
}
