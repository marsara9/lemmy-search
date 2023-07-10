pub mod author;
pub mod community;
pub mod id;
pub mod posts;
pub mod site;
pub mod word;
pub mod xref;

use std::collections::{
    HashSet, 
    HashMap
};
use postgres::types::ToSql;

pub trait DatabaseSchema {

    fn get_table_name(

    ) -> String;

    fn get_keys(

    ) -> Vec<String> {
        vec![Self::get_column_names().first().unwrap().to_string()]
    }

    fn get_column_names(

    ) -> Vec<String>;

    fn get_column_types(

    ) -> HashMap<String, DatabaseType>;

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)>;
}

impl<T> DatabaseSchema for Vec<T> where T : DatabaseSchema {
    fn get_table_name(
    
    ) -> String {
        T::get_table_name()    
    }

    fn get_keys(

    ) -> Vec<String> {
        T::get_keys()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        T::get_column_names()    
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        T::get_column_types()    
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        self.into_iter().flat_map(|object| {
            object.get_values()
        }).collect::<Vec<_>>()
    }
}

impl<T> DatabaseSchema for HashSet<T> where T : DatabaseSchema {
    fn get_table_name(
    
    ) -> String {
        T::get_table_name()    
    }

    fn get_keys(

    ) -> Vec<String> {
        T::get_keys()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        T::get_column_names()    
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        T::get_column_types()    
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        self.into_iter().flat_map(|object| {
            object.get_values()
        }).collect::<Vec<_>>()
    }
}

#[allow(unused)]
#[derive(Clone)]
pub enum DatabaseType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    String(i16),
    Uuid,
    DateTime,
    Optional(Box<DatabaseType>),
    Required(Box<DatabaseType>),
    Unique(Box<DatabaseType>),
    DefaultValue(Box<DatabaseType>, String)
}

#[allow(unused)]
impl DatabaseType {
    pub fn to_sql_type_name(
        &self
    ) -> String {
        match self {
            DatabaseType::Bool => "BOOL".to_string(),
            DatabaseType::I8 => "CHAR".to_string(),
            DatabaseType::I16 => "INT2".to_string(),
            DatabaseType::I32 => "INT4".to_string(),
            DatabaseType::I64 => "INT8".to_string(),
            DatabaseType::String(n) => {
                if n > &0 {
                    format!("VARCHAR({})", n)
                } else {
                    "VARCHAR".to_string()
                }
            },
            DatabaseType::Uuid => "UUID".to_string(),
            DatabaseType::DateTime => "TIMESTAMP WITH TIME ZONE".to_string(),
            DatabaseType::Optional(type_) => {
                format!("{} NULL", type_.to_sql_type_name())
            },
            DatabaseType::Required(type_) => {
                format!("{} NOT NULL", type_.to_sql_type_name())
            },
            DatabaseType::Unique(type_) => {
                format!("{} UNIQUE", type_.to_sql_type_name())
            },
            DatabaseType::DefaultValue(type_, value) => {
                format!("{} DEFAULT {}", type_.to_sql_type_name(), value)
            }
        }
    }

    pub fn get_default_value(
        &self
    ) -> Box<&(dyn ToSql + Sync)> {
        match self {
            DatabaseType::Bool => Box::new(&false),
            DatabaseType::I8 => Box::new(&0i8),
            DatabaseType::I16 => Box::new(&0i16),
            DatabaseType::I32 => Box::new(&0i32),
            DatabaseType::I64 => Box::new(&0i64),
            DatabaseType::String(_) => Box::new(&""),
            DatabaseType::Uuid => Box::new(&"00000000-0000-0000-0000-000000000000"),
            DatabaseType::DateTime => Box::new(&"1970-01-01T00:00:00Z"),
            DatabaseType::Optional(type_) => type_.get_default_value(),
            DatabaseType::Required(type_) => type_.get_default_value(),
            DatabaseType::Unique(type_) => type_.get_default_value(),
            DatabaseType::DefaultValue(type_, _) => type_.get_default_value()
        }
    }

    pub fn not_null(
        self
    ) -> DatabaseType {
        DatabaseType::Required(Box::new(self))
    }

    pub fn nullable(
        self
    ) -> DatabaseType {
        DatabaseType::Optional(Box::new(self))
    }

    pub fn unique(
        self
    ) -> DatabaseType {
        DatabaseType::Unique(Box::new(self))
    }

    pub fn default_value(
        self,
        value : String
    ) -> DatabaseType {
        DatabaseType::DefaultValue(Box::new(self), value)
    }
}
