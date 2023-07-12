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
    pub posts : Option<i32>,
    pub comments : Option<i32>,
    pub communities : Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FederatedInstancesRequest;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FederatedInstancesResponse {
    pub federated_instances : FederatedInstances
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FederatedInstances {
    pub linked : Vec<Instance>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Instance {
    pub domain : String,
    pub software : Option<String>
}
