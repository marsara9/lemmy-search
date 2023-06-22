pub mod site;
pub mod comment;
pub mod community;
pub mod post;
pub mod word;
pub mod search;

use async_trait::async_trait;
use super::DatabasePool;
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

    fn get_object_name(&self) -> &str;

    async fn create_table_if_not_exists(
        &self
    ) -> bool;

    async fn drop_table_if_exists(
        &self
    ) -> bool;

    async fn create(
        &self,
        object : T
    ) -> bool;

    async fn retrieve(
        &self, 
        ap_id : &str
    ) -> Option<T>;

    async fn update(
        &self, 
        object : T
    ) -> bool;

    async fn delete(
        &self, 
        ap_id : &str
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
