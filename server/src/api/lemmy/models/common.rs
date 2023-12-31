use serde::{
    Serialize, 
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SortType {
    New,
    Old,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ListingType {
    All,
    Subscribed,
    Local
}
