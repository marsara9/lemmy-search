use reqwest::Result;
mod crawler;
pub mod rest;

#[tokio::main]
async fn main() -> Result<()> {

    let cralwer = crawler::Crawler {
        instance: "voyager.lemmy.ml".to_string()
    };

    // let communities = cralwer.fetch_all_communities()
    //     .await;

    // print!("{:?}", &communities);

    Ok(())
}
