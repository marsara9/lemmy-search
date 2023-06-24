use reqwest::Client;
use serde::{
    Serialize, 
    de::DeserializeOwned
};

use crate::error::LemmySearchError;

pub async fn fetch_json<T: Serialize + Sized, R: Default + DeserializeOwned>(
    url : &str,
    params : T
) -> Result<R, LemmySearchError> {
    let client = Client::new();
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
