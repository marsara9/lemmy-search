pub mod dbo;
pub mod schema;
pub mod migrations;

use crate::{
    config::{
        Postgres, 
        Config
    }, 
    database::{
        migrations::DatabaseMigrations,
        schema::{
            site::Site,
            word::Word, 
            xref::Search, 
            posts::Post
        }
    }, 
    error::{
        Result, 
        LemmySearchError, 
        LogError
    }, 
    api::lemmy::models::{
        author::Author, 
        community::Community, 
        id::LemmyId
    }
};
use deadpool_r2d2::Runtime;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;

use self::schema::DatabaseSchema;

pub type DatabasePool = deadpool_r2d2::Pool<PgManager>;

pub type PgManager = deadpool_r2d2::Manager<
    r2d2_postgres::PostgresConnectionManager<r2d2_postgres::postgres::NoTls>,
>;

#[derive(Clone)]
pub struct Context {
    pub config : Config,
    pub pool : DatabasePool
}

#[derive(Clone)]
pub struct Database {
    pub context : Context
}

impl Database {

    pub async fn create(
        config : &Config
    ) -> Result<Self> {
        Self::create_database_pool(&config.postgres)
            .await
            .map(|pool| {
                Database {
                    context : Context { 
                        config : config.clone(),
                        pool
                     }
                }
            })
    }

    async fn create_database_pool(
        config : &Postgres
    ) -> Result<DatabasePool> {
        let db_config = postgres::Config::new()
            .user(&config.user)
            .password(&config.password)
            .host(&config.hostname)
            .port(config.port)
            .dbname(&config.database)
            .to_owned();

        let r2d2_manager = PostgresConnectionManager::new(
            db_config, NoTls
        );

        let manager = PgManager::new(r2d2_manager, Runtime::Tokio1);
        DatabasePool::builder(manager)
            .max_size(config.max_size)
            .build()
            .map_err(|err| {
                LemmySearchError::from(err)
            })
    }

    pub async fn init_database(
        &self,
    ) -> Result<()> {
        println!("Creating database...");

        let drop_table = false;

        self.create_table_from_schema::<Site>(drop_table)
            .await?;
        self.create_table_from_schema::<Author>(drop_table)
            .await?;
        self.create_table_from_schema::<Community>(drop_table)
            .await?;
        self.create_table_from_schema::<Post>(drop_table)
            .await?;
        self.create_table_from_schema::<LemmyId>(drop_table)
            .await?;
        self.create_table_from_schema::<Word>(drop_table)
            .await?;
        self.create_table_from_schema::<Search>(drop_table)
            .await?;

        println!("Performing table migrations...");

        let database_migrations = DatabaseMigrations::new(self.context.clone());
        database_migrations.update_table_columns()
            .await?;

        println!("...done");

        Ok(())
    }

    async fn create_table_from_schema<S : DatabaseSchema>(
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

        let log: bool = self.context.config.postgres.log;

        let client = self.context.pool.get()
            .await?;

        client.interact(move |client| -> Result<()> {
            if drop {
                client.execute(&drop_table, &[])?;
            }

            client.execute(&create_table, &[]).map(|_| {
                ()
            }).map_err(|err| {
                LemmySearchError::Database(err)
            }).log_error(format!("...table creation failed for table '{}'", S::get_table_name()).as_str(), log)
        }).await?
    }
}
