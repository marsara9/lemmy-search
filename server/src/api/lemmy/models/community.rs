use serde::{
    Serialize, 
    Deserialize
};

use super::common::SortType;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommunityListRequest {
  pub sort: Option<SortType>,
  pub limit: i64,
  pub page : i64
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
    pub name : String,
    pub title : String
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Counts {
    pub posts : i64
}
