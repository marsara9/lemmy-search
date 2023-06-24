use reqwest::{
    Client,
    Error
};
use serde::{
    Serialize, 
    de::DeserializeOwned
};

pub async fn fetch_json<T: Serialize + Sized, R: Default + DeserializeOwned>(
    url : &str,
    params : T
) -> Result<R, Error> {
    let client = Client::new();
    return match client
        .get(url)
        .query(&params)
        .send()
        .await {
            Ok(response) => {
                println!("Got responsse: {}", &response.text().await.unwrap_or_default());
                Ok(Default::default())
                // response.json()
                //     .await
            }
            Err(err) => {
                Err(err)
            }
        }
}
