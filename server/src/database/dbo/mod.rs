pub mod site;
pub mod comment;
pub mod community;
pub mod post;
pub mod word;
pub mod search;

use async_trait::async_trait;
use crate::error::LemmySearchError;

use super::DatabasePool;
use postgres::{
    NoTls
};
use std::thread;
use r2d2_postgres::{
    r2d2::PooledConnection, 
    PostgresConnectionManager
};

#[async_trait]
pub trait DBO<T : Default> {

    fn get_object_name(&self) -> &str;

    async fn create_table_if_not_exists(
        &self
    ) -> Result<(), LemmySearchError>;

    async fn drop_table_if_exists(
        &self
    ) -> Result<(), LemmySearchError>;

    async fn retrieve(
        &self, 
        ap_id : &str
    ) -> Result<T, LemmySearchError>;

    async fn upsert(
        &self,
        object : T
    ) -> Result<bool, LemmySearchError>;
}

fn get_database_client<T, F>(
    pool : &DatabasePool,
    callback : F
) -> Result<T, LemmySearchError> 
where
    F : FnOnce(&mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<T, postgres::Error> + Send + 'static,
    T : Default + Send + 'static
{
    let pool = pool.clone();

    thread::spawn(move || -> Result<T, LemmySearchError> {
        let mut client = pool.get()
            .map_err(|err| {
                LemmySearchError::DatabaseConnection(err)
            })?;

        callback(&mut client).map_err(|err| {
            LemmySearchError::Database(err)
        })
    }).join().map_err(|_| {
        LemmySearchError::Unknown
    })?
}
