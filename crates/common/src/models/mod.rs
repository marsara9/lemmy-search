use serde::{
    Serialize, 
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LemmyInstance {
    pub name : String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LemmyPost {
    pub title : String,
    pub up_votes : i64,
    pub instance : LemmyInstance,
    pub comments : Vec<LemmyComment>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LemmyComment {
    pub body : String,
    pub up_votes : i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchQuery {
    pub query : String,
    pub page : Option<i64>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub original_query : SearchQuery,
    pub search_results : Vec<LemmyPost>,
    pub total_pages : i64
}
