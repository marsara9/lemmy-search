use reqwest::Result;
mod api;

#[tokio::main]
async fn main() -> Result<()> {

    let communities = api::crawl::fetch_all_communities(
        "voyager.lemmy.ml"
    ).await;

    print!("{:?}", &communities);

    Ok(())
}
