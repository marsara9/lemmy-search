pub mod site;
pub mod comment;
pub mod post;

use async_trait::async_trait;
use super::DatabasePool;
use uuid::Uuid;
use postgres::NoTls;
use std::{
    thread, 
    any::Any, borrow::BorrowMut
};
use r2d2_postgres::{
    r2d2::PooledConnection, 
    PostgresConnectionManager
};

#[async_trait]
pub trait DBO<T : Default> {

    async fn create_table_if_not_exists(
        &self
    ) -> bool;

    async fn create(
        &self, 
        instance : &str,
        object : &T
    ) -> bool;

    async fn retrieve(
        &self, 
        remote_id : &i64,
        instance : &str
    ) -> Option<T>;

    async fn update(
        &self, 
        remote_id : &i64,
        instance : &str
    ) -> bool;

    async fn delete(
        &self, 
        remote_id : &i64,
        instance : &str
    ) -> bool;
}

async fn get_database_client<T, F>(
    pool : &DatabasePool,
    callback : F
) -> Result<T, Box<(dyn Any + Send + 'static)>> 
where 
    F : FnOnce(&mut PooledConnection<PostgresConnectionManager<NoTls>>) -> T,
    F : Send + 'static,
    T : Send + 'static
{
    let pool = pool.clone();
    thread::spawn(move || {
        let mut client = pool.get().unwrap();

        callback(client.borrow_mut())
    }).join()
}
