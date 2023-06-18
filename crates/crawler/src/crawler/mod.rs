mod fetcher;
mod analyizer;

use async_trait::async_trait;
use lemmy_api_common::{
    lemmy_db_schema::{
        newtypes::{CommunityId, PostId},
    }, 
    post::{
        GetPostsResponse
    }, 
    comment::{
        GetCommentsResponse
    }, 
    site::{
        GetFederatedInstancesResponse, 
        SiteResponse, 
    }, 
    lemmy_db_views_actor::structs::CommunityView
};

pub struct Crawler {
    pub instance : String
}

#[async_trait]
pub trait Fetcher {

    const DEFAULT_LIMIT : i64 = 50;

    async fn fetch_site_data(
        &self
    ) -> SiteResponse;

    async fn fetch_instances(
        &self
    ) -> GetFederatedInstancesResponse;

    async fn fetch_all_communities(
        &self,
        number_of_communities: i64
    ) -> Vec<CommunityView>;

    async fn fetch_comments(
        &self,
        post_id : PostId,
        page: i64
    ) -> GetCommentsResponse;

    async fn fetch_posts(
        &self,
        community_id : CommunityId,
        page: i64
    ) -> GetPostsResponse;
}

impl Crawler {
    fn get_url(
        &self,
        path : &str
    ) -> String {
        return format!("https://{}{}", self.instance, path);
    }

    pub async fn crawl(&self) {
        let site_data = self.fetch_site_data()
            .await.site_view;

        let number_of_communities = site_data.counts.communities;

        let _ = self.fetch_all_communities(number_of_communities);
    }
}
