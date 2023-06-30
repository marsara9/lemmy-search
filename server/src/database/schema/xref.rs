use std::{
    hash::Hash, 
    collections::HashMap
};
use postgres::types::ToSql;
use uuid::Uuid;
use super::{
    DatabaseSchema, 
    DatabaseType
};

#[derive(Debug)]
pub struct Search {
    pub word_id : Uuid,
    pub post_ap_id : String
}

impl DatabaseSchema for Search {

    fn get_table_name(

    ) -> String {
        "xref".to_string()
    }

    fn get_keys(
    
    ) -> Vec<String> {
        Self::get_column_names()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "word_id".to_string(),
            "post_ap_id".to_string(),
        ]
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        HashMap::from([
            ("word_id".to_string(), DatabaseType::Uuid.not_null()),
            ("post_ap_id".to_string(), DatabaseType::String(0).not_null()),
        ])
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.word_id,
            &self.post_ap_id
        ]
    }
}

impl PartialEq for Search {
    fn eq(&self, other: &Self) -> bool {
        self.word_id == other.word_id && self.post_ap_id == other.post_ap_id
    }
}

impl Eq for Search {

}

impl Hash for Search {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.word_id.hash(state);
        self.post_ap_id.hash(state);
    }
}
