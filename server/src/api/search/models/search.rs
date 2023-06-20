use serde::{
    Serialize, 
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchQuery {
    pub query : String,
    pub page : Option<i64>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchResult {
    pub original_query : SearchQuery,
    pub search_results : Vec<SearchPost>,
    pub total_pages : i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchPost {
    pub name : String,
    pub body : Option<String>,
    pub score : i64,
    pub comments : Vec<SearchComment>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchComment {
    pub content : String,
    pub score : i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchInstance {
    pub instance : String,
    pub name : String,
    pub actor_id : String
}
