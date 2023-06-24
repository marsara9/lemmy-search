use serde::{
    Serialize, 
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SiteRequest;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SiteResponse {
    pub site_view : SiteView
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SiteView {
    pub site : Site,
    pub local_site_rate_limit : Option<LocalSiteRateLimit>,
    pub counts : Counts
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Site {
    pub name : String,
    pub actor_id : String
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LocalSiteRateLimit {

}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Counts {
    pub posts : Option<i64>,
    pub comments : Option<i64>,
    pub communities : Option<i64>
}