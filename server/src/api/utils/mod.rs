use reqwest::Client;
use serde::{
    Serialize, 
    de::DeserializeOwned
};

pub async fn fetch_json<T: Serialize + ?Sized, R: DeserializeOwned>(
    url : String,
    params : Box<T>
) -> R {
    let client = Client::new();
    return client
        .get(url)
        .query(params.as_ref())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
}
