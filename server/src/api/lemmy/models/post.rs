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
    pub community_id : Option<i32>,
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
    pub community : Community,
    pub counts : Counts
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Post {
    pub id : i32,
    pub name : String,
    pub body : Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Counts {
    pub comments : i64,
    pub score : i32
}
