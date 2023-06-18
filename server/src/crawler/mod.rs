mod analyizer;

pub struct Crawler {
    pub instance : String
}

impl Crawler {
    fn get_url(
        &self,
        path : &str
    ) -> String {
        return format!("https://{}{}", self.instance, path);
    }

    // pub async fn crawl(&self) {
    //     let site_data = self.fetch_site_data()
    //         .await.site_view;

    //     let number_of_communities = site_data.counts.communities;

    //     let _ = self.fetch_all_communities(number_of_communities);
    // }
}
