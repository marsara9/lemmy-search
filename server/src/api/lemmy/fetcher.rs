use crate::api::utils::fetch_json;
use reqwest::Error;
use serde::{
    de::DeserializeOwned, 
    Serialize
};
use super::models::{
    common::SortType,
    site::{
        SiteRequest,
        SiteResponse
    },
    community::{
        CommunityListResponse, 
        CommunityListRequest, 
        CommunityData
    }, 
    post::{
        PostData, 
        PostListRequest, 
        PostListResponse
    }, 
    comment::{
        CommentListRequest, 
        CommentListResponse, 
        CommentData
    }
};

pub struct Fetcher {
    instance : String
}

#[allow(unused)]
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
        number_of_communities : i64,
    ) -> Result<Vec<CommunityData>, Error> {
        self.fetch_multiple(
            "/api/v3/community/list", 
            number_of_communities, 
            |index| {
                CommunityListRequest {
                    sort: Some(SortType::Old),
                    limit: Self::DEFAULT_LIMIT,
                    page: index
                }
            }, 
            |response : CommunityListResponse| {
                response.communities
            }
        ).await
    }

    pub async fn fetch_posts(
        &self,
        community_id: i64,
        number_of_posts : i64,
    ) -> Result<Vec<PostData>, Error> {
        self.fetch_multiple(
            "/api/v3/post/list", 
            number_of_posts, 
            |index| {
                PostListRequest {
                    community_id : Some(community_id),
                    sort: Some(SortType::Old),
                    limit: Self::DEFAULT_LIMIT,
                    page: index
                }
            }, 
            |response : PostListResponse| {
                response.posts
            }
        ).await
    }

    pub async fn fetch_comments(
        &self,
        post_id: i64,
        number_of_comments : i64,
    ) -> Result<Vec<CommentData>, Error> {
        self.fetch_multiple(
            "/api/v3/comment/list", 
            number_of_comments, 
            |index| {
                CommentListRequest {
                    post_id : Some(post_id),
                    sort: Some(SortType::Old),
                    limit: Self::DEFAULT_LIMIT,
                    page: index
                }
            }, 
            |response : CommentListResponse| {
                response.comments
            }
        ).await
    }

    async fn fetch_multiple<PF, P, R, RM, M>(
        &self,
        path : &str,
        total_items : i64,
        params_creator : PF,
        result_mapper : RM
    ) -> Result<Vec<M>, Error>
    where
        PF : Fn(i64) -> P,
        P : Serialize + Sized,
        R : DeserializeOwned,
        RM : Fn(R) -> Vec<M>,
        M : DeserializeOwned + Clone
    {
        let number_of_calls = total_items / Self::DEFAULT_LIMIT;

        let url = self.get_url(path);
        let calls = (0..number_of_calls).map(params_creator)
            .map(|params| {
                fetch_json::<P, R>(&url, params)
            });

        let results = futures::future::join_all(calls)
            .await;

        let mut response = Vec::<M>::new();
        for result in results { 
            match result {
                Ok(value) => {
                    let items = result_mapper(value);
                    for item in items {
                        response.push(item.clone());
                    }
                },
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(response)
    }
}
