use serde::{
    Serialize, 
    Deserialize
};
use crate::models::LemmyPost;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page {
    pub word : String,
    pub posts : Vec<LemmyPost>
}
