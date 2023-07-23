
use serde::{
    Serialize, 
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Redirect {
    pub actor_id : String,
    pub home_instance : String
}
