pub mod dbo;
pub mod schema;

use crate::{
    config::Postgres, 
    database::dbo::{
        DBO, 
        author::AuthorDBO, 
        comment::CommentDBO, 
        community::CommunityDBO, 
        id::IdDBO,
        post::PostDBO, 
        search::SearchDatabase, 
        site::SiteDBO, 
        word::WordsDBO
    }, 
    error::{
        LemmySearchError,
        LogError
    }
};
use postgres::{
    NoTls, 
    Config
};
use r2d2_postgres::{
    PostgresConnectionManager, 
    r2d2::{Pool, PooledConnection}
};

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
    ) -> Result<Self, r2d2_postgres::r2d2::Error> {
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
    ) -> Result<DatabasePool, r2d2_postgres::r2d2::Error> {
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

    pub async fn init_database(
        &self,
    ) -> Result<(), LemmySearchError> {
        println!("Creating database...");

        self.create_table(
            SiteDBO::new(self.pool.clone())
        ).await?;
        self.create_table(
            AuthorDBO::new(self.pool.clone())
        ).await?;
        self.create_table(
            CommunityDBO::new(self.pool.clone())
        ).await?;
        self.create_table(
            PostDBO::new(self.pool.clone())
        ).await?;
        self.create_table(
            CommentDBO::new(self.pool.clone())
        ).await?;
        self.create_table(
            IdDBO::new(self.pool.clone())
        ).await?;
        self.create_table(
            WordsDBO::new(self.pool.clone())
        ).await?;

        println!("\tCreating SEARCH table...");
        let search = SearchDatabase::new(self.pool.clone());
        // search.drop_table_if_exists()
            // .await?;
        search.create_table_if_not_exists()
            .await
            .log_error("\t\t...failed to create table.", self.config.log)?;

        Ok(())
    }

    async fn create_table<D, T>(
        &self,
        dbo : D
    ) -> Result<(), LemmySearchError>
    where 
        D : DBO<T> + Sized,
        T : Default
    {
        println!("\tCreating '{}' table...", dbo.get_object_name());
        // dbo.drop_table_if_exists()
            // .await?;
        dbo.create_table_if_not_exists()
            .await.log_error("\t\t...failed to create table.", self.config.log)
    }
}
