use crate::api::utils::fetch_json;
use reqwest::Error;
use super::models::{
    common::SortType,
    site::{
        SiteRequest,
        SiteResponse
    },
    community::{
        CommunityListRequest, 
        CommunityData
    }, 
    post::{
        PostData, 
        PostListRequest, 
    }, 
    comment::{
        CommentListRequest, 
        CommentData
    }
};

pub struct Fetcher {
    instance : String
}

impl Fetcher {

    pub const DEFAULT_LIMIT : i64 = 50;

    pub fn new(instance : String) -> Self {
        Self {
            instance
        }
    }

    fn get_url(
        &self,
        path : &str
    ) -> String {
        return format!("https://{}{}", self.instance, path);
    }

    pub async fn fetch_site_data(
        &self
    ) -> Result<SiteResponse, Error> {
        let params = SiteRequest;
        let url = self.get_url("/api/v3/site");
        fetch_json::<SiteRequest, SiteResponse>(&url, params)
            .await
    }

    pub async fn fetch_communities(
        &self,
        page : i64
    ) -> Result<Vec<CommunityData>, Error> {
        let params = CommunityListRequest {
            sort: Some(SortType::Old),
            limit: Self::DEFAULT_LIMIT,
            page: page
        };

        let url = self.get_url("/api/v3/community/list");

        fetch_json(&url, params)
            .await
    }

    pub async fn fetch_posts(
        &self,
        page : i64
    ) -> Result<Vec<PostData>, Error> {
        let params = PostListRequest {
            sort: Some(SortType::Old),
            limit: Self::DEFAULT_LIMIT,
            page: page,
            ..Default::default()
        };

        let url = self.get_url("/api/v3/post/list");

        fetch_json(&url, params)
            .await
    }

    pub async fn fetch_comments(
        &self,
        page : i64
    ) -> Result<Vec<CommentData>, Error> {
        let params = CommentListRequest {
            sort: Some(SortType::Old),
            limit: Self::DEFAULT_LIMIT,
            page: page,
            ..Default::default()
        };

        let url = self.get_url("/api/v3/comment/list");

        fetch_json(&url, params)
            .await
    }
}
