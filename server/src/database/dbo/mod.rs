pub mod site;
pub mod search;
pub mod crawler;

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

pub fn get_database_client<T, F>(
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
