use std::fmt::Debug;

use reqwest::Client;
use serde::{
    Serialize, 
    de::DeserializeOwned
};

use crate::error::LemmySearchError;

pub async fn fetch_json<T, R>(
    url : &str,
    params : T
) -> Result<R, LemmySearchError>
where
    T : Serialize + Sized + Debug,
    R : Default + DeserializeOwned
{
    let client = Client::builder()
        .connection_verbose(true)
        .build()?;

    println!("Connecting to {}...", url);
    println!("\twith params {:?}...", params);

    return match client
        .get(url)
        .query(&params)
        .send()
        .await {
            Ok(response) => {
                response.json()
                    .await.map_err(|err| {
                        LemmySearchError::Network(err)
                    })
            }
            Err(err) => {
                Err(LemmySearchError::Network(err))
            }
        }
}
