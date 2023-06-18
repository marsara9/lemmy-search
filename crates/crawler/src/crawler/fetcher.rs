use super::{Crawler, Fetcher};
use async_trait::async_trait;
use crate::rest;
use futures::FutureExt;

use lemmy_api_common::{
    community::{
        ListCommunities, 
        ListCommunitiesResponse
    }, 
    lemmy_db_schema::{
        ListingType, 
        SortType, newtypes::{CommunityId, PostId}, CommentSortType
    }, 
    post::{
        GetPosts, 
        GetPostsResponse
    }, 
    comment::{
        GetCommentsResponse, 
        GetComments
    }, 
    site::{
        GetFederatedInstances, 
        GetFederatedInstancesResponse, 
        SiteResponse, 
        GetSite
    }, 
    lemmy_db_views_actor::structs::CommunityView
};

#[async_trait]
impl Fetcher for Crawler {

    async fn fetch_site_data(
        &self
    ) -> SiteResponse {
        let params = GetSite {
            auth: None
        };

        let url = self.get_url("/api/v3/site");
        return rest::fetch_json::<GetSite, SiteResponse>(url, Box::new(params))
            .await;
    }

    async fn fetch_instances(
        &self
    ) -> GetFederatedInstancesResponse {
        let params = GetFederatedInstances {
            auth: None
        };

        let url = self.get_url("/api/v3/federated_instances");
        return rest::fetch_json::<GetFederatedInstances, GetFederatedInstancesResponse>(url, Box::new(params))
            .await;
    }
    
    async fn fetch_comments(
        &self,
        post_id : PostId,
        page: i64
    ) -> GetCommentsResponse {
        let params = GetComments {
            post_id: Some(post_id),
            page: Some(page),
            sort: Some(CommentSortType::Old),
            limit: Some(Self::DEFAULT_LIMIT),
            ..Default::default()
        };
    
        let url = self.get_url("/api/v3/comment/list");
        return rest::fetch_json::<GetComments, GetCommentsResponse>(url, Box::new(params))
            .await;
    }
    
    async fn fetch_posts(
        &self,
        community_id : CommunityId,
        page: i64
    ) -> GetPostsResponse {
        let params = GetPosts {
            community_id: Some(community_id),
            page: Some(page),
            sort: Some(SortType::Old),
            limit: Some(Self::DEFAULT_LIMIT),
            ..Default::default()
        };
    
        let url = self.get_url("/api/v3/post/list");
        return rest::fetch_json::<GetPosts, GetPostsResponse>(url, Box::new(params))
            .await;
    }
    
    async fn fetch_all_communities(
        &self,
        number_of_communities : i64
    ) -> Vec<CommunityView> {

        let number_of_calls = number_of_communities / Self::DEFAULT_LIMIT;

        let url = self.get_url("/api/v3/community/list");
        
        let calls = (0..number_of_calls).map(|index|
            ListCommunities {
                type_: Some(ListingType::Local),
                sort: Some(SortType::Old),
                limit: Some(Self::DEFAULT_LIMIT),
                page: Some(index),
                ..Default::default()
            }
        ).map(|params|
            rest::fetch_json::<ListCommunities, ListCommunitiesResponse>(url.to_owned(), Box::new(params)).boxed()
        ).map(async_std::task::spawn)
            .collect::<Vec<async_std::task::JoinHandle<ListCommunitiesResponse>>>();

        let results = futures::future::join_all(calls).await.iter().map(|list|
            list.to_owned().communities
        ).flatten().collect();

        results
    }
}
