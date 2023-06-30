use std::{
    hash::Hash, 
    collections::HashMap
};
use postgres::types::ToSql;
use crate::api::lemmy::models::id::LemmyId;
use super::{
    DatabaseSchema, 
    DatabaseType
};

impl DatabaseSchema for LemmyId {

    fn get_table_name(

    ) -> String {
        "lemmy_ids".to_string()
    }

    fn get_keys(
    
    ) -> Vec<String> {
        vec![
            "post_actor_id".to_string(),
            "instance_actor_id".to_string() 
        ]    
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "post_remote_id".to_string(),
            "post_actor_id".to_string(),
            "instance_actor_id".to_string()
        ]
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        HashMap::from([
            ("post_remote_id".to_string(), DatabaseType::I64.not_null()),
            ("post_actor_id".to_string(), DatabaseType::String(0).not_null()),
            ("instance_actor_id".to_string(), DatabaseType::String(0).not_null())
        ])
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.post_remote_id,
            &self.post_actor_id,
            &self.instance_actor_id
        ]
    }
}

impl PartialEq for LemmyId {
    fn eq(&self, other: &Self) -> bool {
        self.post_actor_id == other.post_actor_id && self.instance_actor_id == other.instance_actor_id
    }
}

impl Eq for LemmyId {

}

impl Hash for LemmyId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.post_actor_id.hash(state);
        self.instance_actor_id.hash(state);
    }
}
