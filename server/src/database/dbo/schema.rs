use std::collections::HashSet;
use postgres::types::ToSql;

pub trait DatabaseSchema {

    fn get_table_name(

    ) -> String;

    fn get_keys(

    ) -> Vec<String> {
        vec![Self::get_column_names().first().unwrap().to_string()]
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)>;

    fn get_column_names(

    ) -> Vec<String>;
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

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        self.into_iter().flat_map(|object| {
            object.get_values()
        }).collect::<Vec<_>>()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        T::get_column_names()    
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

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        self.into_iter().flat_map(|object| {
            object.get_values()
        }).collect::<Vec<_>>()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        T::get_column_names()    
    }
}

