pub mod dbo;

use crate::{
    config::Postgres, 
    database::dbo::{
        comment::CommentDBO, 
        DBO, site::SiteDBO, 
        post::PostDBO, 
        word::WordsDBO, 
        community::CommunityDBO, search::SearchDatabase
    }
};
use postgres::{
    NoTls, 
    Config,
    Error
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

    pub async fn create(
        config : &Postgres
    ) -> Result<Self, r2d2_postgres::r2d2::Error> {
        Self::create_database_pool(config)
            .await
            .map(|pool| {
                Database {
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
    ) -> Result<(), Error> {
        println!("Creating database...");

        println!("\tCreating COMMUNITIES table...");
        let communities = CommunityDBO::new(self.pool.clone());
        communities.drop_table_if_exists()
            .await;
        if !communities.create_table_if_not_exists()
            .await {
                println!("\t\t...failed to create table.");
            }

        println!("\tCreating POSTS table...");
        let post = PostDBO::new(self.pool.clone());
        post.drop_table_if_exists()
            .await;
        if !post.create_table_if_not_exists()
            .await {
                println!("\t\t...failed to create table.");
            }

        println!("\tCreating COMMENTS table...");
        let comment = CommentDBO::new(self.pool.clone());
        comment.drop_table_if_exists()
            .await;
        if !comment.create_table_if_not_exists()
            .await {
                println!("\t\t...failed to create table.");
            }

        println!("\tCreating SITES table...");
        let site = SiteDBO::new(self.pool.clone());
        site.drop_table_if_exists()
            .await;
        if !site.create_table_if_not_exists()
            .await {
                println!("\t\t...failed to create table.");
            }

        println!("\tCreating WORDS table...");
        let word = WordsDBO::new(self.pool.clone());
        word.drop_table_if_exists()
            .await;
        if !word.create_table_if_not_exists()
            .await {
                println!("\t\t...failed to create table.");
            }

        println!("\tCreating SEARCH table...");
        let search = SearchDatabase::new(self.pool.clone());
        search.drop_table_if_exists()
            .await;
        if !search.create_table_if_not_exists()
            .await {
                println!("\t\t...failed to create table.");
            }

        Ok(())
    }
}
