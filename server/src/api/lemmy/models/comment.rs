use serde::{
    Serialize, 
    Deserialize
};

use super::{
    common::SortType, 
    post::{Post, Creator}, 
    community::Community
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommentListRequest {
    pub post_id : Option<i64>,
    pub sort : Option<SortType>,
    pub limit : i64,
    pub page : i64
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommentListResponse {
    pub comments : Vec<CommentData>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommentData {
    pub comment : Comment,
    pub creator : Creator,
    pub post : Post,
    pub community : Community,
    pub counts : Counts
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Comment {
    pub ap_id : String,
    pub content : String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Counts {
    pub score : Option<i32>
}
