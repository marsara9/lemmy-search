use std::{time::Duration, collections::HashSet};

use chrono::{DateTime, Utc};
use serde::{
    Serialize, 
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Version {
    pub version : String
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchQuery {
    pub query : String,
    pub home_instance : String,
    pub page : Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchResult {
    pub original_query_terms : HashSet<String>,
    pub total_results : i32,
    pub total_pages : i32,
    pub time_taken : Duration,
    pub posts : Vec<SearchPost>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchPost {
    pub url : Option<String>,
    pub name : String,
    pub body : Option<String>,
    pub updated : DateTime<Utc>,
    pub remote_id : i64,
    pub author : SearchAuthor,
    pub community: SearchCommunity,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchAuthor {
    pub actor_id : String,
    pub avatar : Option<String>,
    pub name : String,
    pub display_name : Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchCommunity {
    pub actor_id : String,
    pub icon : Option<String>,
    pub name : String,
    pub title : Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchInstance {
    pub actor_id : String,
    pub instance : String,
    pub name : String,
}
