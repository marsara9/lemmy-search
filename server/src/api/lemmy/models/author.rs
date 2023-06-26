use serde::{
    Serialize, 
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Author {
    pub actor_id : String,
    pub avatar : Option<String>,
    pub name : String,
    pub display_name : Option<String>
}
