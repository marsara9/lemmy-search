use std::collections::HashMap;
use postgres::types::ToSql;
use super::{
    DatabaseSchema, 
    DatabaseType
};

pub struct Version {
    version_number : i32
}

impl DatabaseSchema for Version {

    fn get_table_name(

    ) -> String {
        "version".to_string()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "version_number".to_string()
        ]
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        HashMap::from([
            ("version_number".to_string(), DatabaseType::I32.not_null())
        ])
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.version_number
        ]
    }
}
