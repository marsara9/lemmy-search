use serde::{
    Serialize, 
    Deserialize
};

use crate::api::lemmy::models::post::Post;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Page {
//     pub word : String,
//     pub posts : Vec<LemmyPost>
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchQuery {
    pub query : String,
    pub page : Option<i64>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub original_query : SearchQuery,
    pub search_results : Vec<Post>,
    pub total_pages : i64
}
