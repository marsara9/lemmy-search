pub mod author;
pub mod site;
pub mod comment;
pub mod community;
pub mod id;
pub mod post;
pub mod word;
pub mod search;
pub mod crawler;
pub mod schema;

use async_trait::async_trait;
use super::DatabasePool;
use std::thread;
use crate::error::{
    Result, 
    LemmySearchError
};
use postgres::NoTls;
use r2d2_postgres::{
    r2d2::PooledConnection, 
    PostgresConnectionManager
};

#[async_trait]
pub trait DBO<T : Default> {

    fn get_object_name(&self) -> &str;

    async fn create_table_if_not_exists(
        &self
    ) -> Result<()>;

    async fn drop_table_if_exists(
        &self
    ) -> Result<()>;
}

fn get_database_client<T, F>(
    pool : &DatabasePool,
    callback : F
) -> Result<T> 
where
    F : FnOnce(&mut PooledConnection<PostgresConnectionManager<NoTls>>) -> std::result::Result<T, postgres::Error> + Send + 'static,
    T : Default + Send + 'static
{
    let pool = pool.clone();

    thread::spawn(move || -> Result<T> {
        let mut client = pool.get()?;

        callback(&mut client).map_err(|err| {
            LemmySearchError::Database(err)
        })
    }).join().map_err(|err| {
        LemmySearchError::Unknown(format!("{:#?}", err))
    })?
}
