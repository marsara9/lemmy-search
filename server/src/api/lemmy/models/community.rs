use serde::{
    Serialize, 
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Community {
    pub actor_id : String,
    pub icon : Option<String>,
    pub name : String,
    pub title : Option<String>
}
