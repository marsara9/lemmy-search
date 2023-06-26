use crate::{
    api::utils::fetch_json, 
    error::LemmySearchError
};
use super::models::{
    common::SortType,
    site::{
        SiteRequest,
        SiteResponse, 
        FederatedInstancesResponse, 
        FederatedInstancesRequest
    },
    post::{
        PostData, 
        PostListRequest, PostListResponse, 
    }, 
    comment::{
        CommentListRequest, 
        CommentData, CommentListResponse
    }
};

pub struct Fetcher {
    instance : String
}

impl Fetcher {

    pub const DEFAULT_LIMIT : i32 = 50;

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
    ) -> Result<SiteResponse, LemmySearchError> {
        let params = SiteRequest;
        let url = self.get_url("/api/v3/site");
        fetch_json::<SiteRequest, SiteResponse>(&url, params)
            .await
    }

    pub async fn fetch_instances(
        &self
    ) -> Result<FederatedInstancesResponse, LemmySearchError> {
        let params = FederatedInstancesRequest;
        let url = self.get_url("/api/v3/federated_instances");
        fetch_json(&url, params)
            .await
    }

    // pub async fn fetch_communities(
    //     &self,
    //     page : i32
    // ) -> Result<Vec<CommunityData>, LemmySearchError> {
    //     let params = CommunityListRequest {
    //         sort: Some(SortType::Old),
    //         limit: Self::DEFAULT_LIMIT,
    //         page: page
    //     };

    //     let url = self.get_url("/api/v3/community/list");

    //     fetch_json(&url, params)
    //         .await
    //         .map(|view: CommunityListResponse| {
    //             view.communities
    //         })
    // }

    pub async fn fetch_posts(
        &self,
        page : i32
    ) -> Result<Vec<PostData>, LemmySearchError> {
        let params = PostListRequest {
            sort: Some(SortType::Old),
            limit: Self::DEFAULT_LIMIT,
            page: page,
            ..Default::default()
        };

        let url = self.get_url("/api/v3/post/list");

        fetch_json(&url, params)
            .await
            .map(|view: PostListResponse| {
                view.posts
            })
    }

    pub async fn fetch_comments(
        &self,
        page : i32
    ) -> Result<Vec<CommentData>, LemmySearchError> {
        let params = CommentListRequest {
            sort: Some(SortType::Old),
            limit: Self::DEFAULT_LIMIT,
            page: page,
            ..Default::default()
        };

        let url = self.get_url("/api/v3/comment/list");

        fetch_json(&url, params)
            .await
            .map(|view:CommentListResponse| {
                view.comments
            })
    }
}
