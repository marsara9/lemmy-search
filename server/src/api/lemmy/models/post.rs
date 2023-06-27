use serde::{
    Serialize, 
    Deserialize
};

use super::{
    common::SortType, 
    community::Community, 
    author::Author
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PostListRequest {
    pub community_id : Option<i64>,
    pub sort : Option<SortType>,
    pub limit : i32,
    pub page : i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PostListResponse {
    pub posts : Vec<PostData>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct  PostData {
    pub post : Post,
    pub creator : Author,
    pub community : Community,
    pub counts : Counts
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Post {
    pub id : i64,
    pub ap_id : String,
    pub url : Option<String>,
    pub name : String,
    pub body : Option<String>,
    pub removed : Option<bool>,
    pub deleted : Option<bool>,
    pub language_id : i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Counts {
    pub comments : i64,
    pub score : i32
}
