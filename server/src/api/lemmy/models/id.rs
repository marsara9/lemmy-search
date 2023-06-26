use serde::{
    Serialize, 
    Deserialize
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LemmyId {
    pub post_remote_id : i64,
    pub post_actor_id : String,
    pub instance_actor_id : String
}

