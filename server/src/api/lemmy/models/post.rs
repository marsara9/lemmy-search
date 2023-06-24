use serde::{
    Serialize, 
    Deserialize
};

use super::{
    common::SortType, 
    community::Community
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PostListRequest {
    pub community_id : Option<i64>,
    pub sort : Option<SortType>,
    pub limit : i64,
    pub page : i64
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PostListResponse {
    pub posts : Vec<PostData>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct  PostData {
    pub post : Post,
    pub creator : Creator,
    pub community : Community,
    pub counts : Counts
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Post {
    pub ap_id : String,
    pub url : Option<String>,
    pub name : String,
    pub body : Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Creator {
    pub actor_id : String
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Counts {
    pub comments : i64,
    pub score : i32
}
