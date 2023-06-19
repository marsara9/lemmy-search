use serde::{
    Serialize, 
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SiteRequest;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SiteResponse {
    pub site_view : SiteView,
    pub federated_instances : FederatedInstances
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SiteView {
    pub local_site_rate_limit : LocalSiteRateLimit,
    pub counts : Counts
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FederatedInstances {
    pub linked : Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LocalSiteRateLimit {

}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Counts {
    pub posts : i64,
    pub comments : i64,
    pub communities : i64
}