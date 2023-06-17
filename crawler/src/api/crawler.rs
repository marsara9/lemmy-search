use serde::{Serialize, de::DeserializeOwned};
use lemmy_api_common::{
    community::{
        ListCommunities, 
        ListCommunitiesResponse
    }, 
    lemmy_db_schema::{
        ListingType, 
        SortType, newtypes::{CommunityId, PostId}, CommentSortType
    }, post::{GetPosts, GetPostsResponse}, comment::{GetCommentsResponse, GetComments}
};
use reqwest::Client;

pub struct Crawler {
    pub instance : String
}

impl Crawler {
    const DEFAULT_LIMIT : i64 = 50;

    fn get_url(
        &self,
        path : &str
    ) -> String {
        return format!("https://{}{}", self.instance, path);
    }
    
    async fn fetch_json<T: Serialize + ?Sized, R: DeserializeOwned>(
        url : String,
        params : &T
    ) -> R {
        let client = Client::new();
        return client
            .get(url)
            .query(&params)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
    }
    
    pub async fn fetch_comments(
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
        return Self::fetch_json::<GetComments, GetCommentsResponse>(url, &params)
            .await;
    }
    
    pub async fn fetch_posts(
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
        return Self::fetch_json::<GetPosts, GetPostsResponse>(url, &params)
            .await;
    }
    
    pub async fn fetch_all_communities(
        self
    ) -> ListCommunitiesResponse {
        let params = ListCommunities {
            type_: Some(ListingType::Local),
            sort: Some(SortType::Old),
            limit: Some(Self::DEFAULT_LIMIT),
            ..Default::default()
        };
    
        let url = self.get_url("/api/v3/community/list");
        return Self::fetch_json::<ListCommunities, ListCommunitiesResponse>(url, &params)
            .await;
    }
}
