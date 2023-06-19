use crate::api::utils::fetch_json;
use async_std::task::JoinHandle;
use futures::FutureExt;
use super::models::{
    common::SortType,
    site::{
        SiteRequest,
        SiteResponse
    },
    comment::{
        CommentListResponse, 
        CommentListRequest, 
        Comment, CommentData
    }
};

pub struct Fetcher {
    instance : String
}

impl Fetcher {

    pub const DEFAULT_LIMIT : i64 = 50;

    pub fn new(instance : String) -> Self {
        Fetcher {
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
    ) -> SiteResponse {
        let params = SiteRequest;

        let url = self.get_url("/api/v3/site");
        return fetch_json::<SiteRequest, SiteResponse>(url, Box::new(params))
            .await;
    }

    pub async fn fetch_all_data(
        &self,
        number_of_comments : Option<i64>
    ) -> Vec<Comment> {

        let number_of_comments = match number_of_comments {
            Some(value) => value,
            None => self.fetch_site_data()
                .await
                .site_view
                .counts
                .comments
        };

        let number_of_calls = number_of_comments / Self::DEFAULT_LIMIT;

        let url = self.get_url("/api/v3/comment/list");
        
        let calls = (0..number_of_calls).map(|index|
            CommentListRequest {
                sort: Some(SortType::Old),
                limit: Self::DEFAULT_LIMIT,
                page: index,
                ..Default::default()
            }
        ).map(|params|
            fetch_json::<CommentListRequest, CommentListResponse>(url.to_owned(), Box::new(params)).boxed()
        ).map(async_std::task::spawn)
            .collect::<Vec<JoinHandle<CommentListResponse>>>();

        let results = futures::future::join_all(calls).await.iter().map(|list|
            list.comments.iter().map(|comment_data|
                comment_data.comment.clone()
            )
        ).flatten().collect();

        results
    }

    pub async fn fetch_comments(
        &self,
        page: i64
    ) -> Vec<CommentData> {
        let url = self.get_url("/api/v3/comment/list");

        let params = CommentListRequest {
            sort: Some(SortType::Old),
            limit: Self::DEFAULT_LIMIT,
            page: page,
            ..Default::default()
        };

        fetch_json::<CommentListRequest, CommentListResponse>(url.to_owned(), Box::new(params))
            .await
            .comments
    }
}