use serde::{
    Serialize, 
    Deserialize
};

use super::{
    common::{
        SortType, 
        ListingType
    }, 
    post::Post, 
    author::Author,
    community::Community
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommentListRequest {
    pub type_ : Option<ListingType>,
    pub post_id : Option<i64>,
    pub sort : Option<SortType>,
    pub limit : i32,
    pub page : i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommentListResponse {
    pub comments : Vec<CommentData>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommentData {
    pub comment : Comment,
    pub creator : Author,
    pub post : Post,
    pub community : Community,
    pub counts : Counts
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Comment {
    pub ap_id : String,
    pub content : String,
    pub deleted : bool,
    pub removed : bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Counts {
    pub score : Option<i32>
}
