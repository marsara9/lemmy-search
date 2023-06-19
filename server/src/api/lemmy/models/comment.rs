use serde::{
    Serialize, 
    Deserialize
};

use super::{
    common::SortType, 
    post::Post, 
    community::Community
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommentListRequest {
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
    pub post : Post,
    pub community : Community,
    pub counts : Counts
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Comment {
    pub id : i64,
    pub content : String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Counts {
    pub score : Option<i32>
}
