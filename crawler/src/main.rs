use reqwest::Result;
mod api;

#[tokio::main]
async fn main() -> Result<()> {

    let cralwer = api::crawler::Crawler {
        instance: "voyager.lemmy.ml".to_string()
    };

    let communities = cralwer.fetch_all_communities()
        .await;

    print!("{:?}", &communities);

    Ok(())
}
