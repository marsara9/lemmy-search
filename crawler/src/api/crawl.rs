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

const DEFAULT_LIMIT : i64 = 50;

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
    instance : &str,
    post_id : PostId,
    page: i64
) -> GetCommentsResponse {
    let params = GetComments {
        post_id: Some(post_id),
        page: Some(page),
        sort: Some(CommentSortType::Old),
        limit: Some(DEFAULT_LIMIT),
        ..Default::default()
    };

    let url = format!("https://{}/api/v3/comment/list", instance);    
    return fetch_json::<GetComments, GetCommentsResponse>(url, &params)
        .await;
}

pub async fn fetch_posts(
    instance : &str,
    community_id : CommunityId,
    page: i64
) -> GetPostsResponse {
    let params = GetPosts {
        community_id: Some(community_id),
        page: Some(page),
        sort: Some(SortType::Old),
        limit: Some(DEFAULT_LIMIT),
        ..Default::default()
    };

    let url = format!("https://{}/api/v3/post/list", instance);    
    return fetch_json::<GetPosts, GetPostsResponse>(url, &params)
        .await;
}

pub async fn fetch_all_communities(
    instance : &str
) -> ListCommunitiesResponse {
    let params = ListCommunities {
        type_: Some(ListingType::Local),
        sort: Some(SortType::Old),
        limit: Some(DEFAULT_LIMIT),
        ..Default::default()
    };

    let url = format!("https://{}/api/v3/community/list", instance);    
    return fetch_json::<ListCommunities, ListCommunitiesResponse>(url, &params)
        .await;
}
