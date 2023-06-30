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
pub struct Word {
    pub id : Uuid,
    pub word : String
}

impl Word {
    pub fn from(word : String) -> Self {
        Self {
            id : Uuid::new_v4(),
            word
        }
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}

impl Eq for Word {

}

impl Hash for Word {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {        
        self.word.hash(state);
    }
}

impl DatabaseSchema for Word {

    fn get_table_name(

    ) -> String {
        "words".to_string()
    }

    fn get_keys(
    
    ) -> Vec<String> {
        Self::get_column_names()    
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "id".to_string(),
            "word".to_string()
        ]
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        HashMap::from([
            ("id".to_string(), DatabaseType::Uuid.not_null()),
            ("word".to_string(), DatabaseType::String(0).not_null()),
        ])
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.id,
            &self.word
        ]
    }
}
