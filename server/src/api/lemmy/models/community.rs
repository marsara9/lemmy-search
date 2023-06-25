use serde::{
    Serialize, 
    Deserialize
};

use super::common::SortType;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommunityListRequest {
  pub sort: Option<SortType>,
  pub limit: i32,
  pub page : i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommunityListResponse {
    pub communities : Vec<CommunityData>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommunityData {
    pub community : Community,
    pub counts : Counts
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Community {
    pub actor_id : String,
    pub icon : Option<String>,
    pub name : String,
    pub title : String
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Counts {
    pub posts : i64
}
