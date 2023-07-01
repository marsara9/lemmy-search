pub mod site;
pub mod search;
pub mod crawler;

use super::DatabasePool;
use crate::error::{
    Result, 
    LemmySearchError
};
use postgres::Client;

pub async fn get_database_client<T, F>(
    pool : &DatabasePool,
    callback : F
) -> Result<T> 
where
    F: FnOnce(&mut Client) -> std::result::Result<T, postgres::Error> + Send + 'static,
    T : Default + Send + 'static
{
    let client = pool.get().await.map_err(|_| {
        LemmySearchError::Unknown("".to_string())
    })?;

    client.interact(move |client| -> Result<T> {
        callback(client).map_err(|err| {
            LemmySearchError::from(err)
        })
    }).await?
}
