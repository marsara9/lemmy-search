use std::{
    hash::Hash, 
    collections::HashMap
};
use postgres::types::ToSql;
use crate::api::lemmy::models::community::Community;
use super::{
    DatabaseSchema, 
    DatabaseType
};

impl DatabaseSchema for Community {

    fn get_table_name(

    ) -> String {
        "communities".to_string()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "ap_id".to_string(),
            "icon".to_string(),
            "name".to_string(),
            "title".to_string()
        ]
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        HashMap::from([
            ("ap_id".to_string(), DatabaseType::String(0).not_null()),
            ("icon".to_string(), DatabaseType::String(0).nullable()),
            ("name".to_string(), DatabaseType::String(0).not_null()),
            ("title".to_string(), DatabaseType::String(0).nullable())
        ])
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.actor_id,
            &self.icon,
            &self.name,
            &self.title
        ]
    }
}

impl PartialEq for Community {
    fn eq(&self, other: &Self) -> bool {
        self.actor_id == other.actor_id
    }
}

impl Eq for Community {

}

impl Hash for Community {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.actor_id.hash(state);
    }
}
