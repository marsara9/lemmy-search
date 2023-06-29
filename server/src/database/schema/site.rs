use std::{
    collections::HashMap, 
    hash::Hash
};
use chrono::{
    DateTime, 
    Utc
};
use postgres::types::ToSql;
use uuid::Uuid;

use super::{
    DatabaseSchema, 
    DatabaseType
};

pub struct Site {
    pub id : Uuid,
    pub name : String,
    pub actor_id : String,
    pub last_post_page : i32,
    pub last_comment_page : i32,
    pub last_update : DateTime<Utc>
}

// impl Site {
//     pub fn fromLemmy(
//         site_view : SiteView
//     ) -> Self {
//         Self {
//             id : Uuid::new_v4(),
//             name : site_view.site.name,
//             actor_id : site_view.site.actor_id,
//             last_post_page : 0,
//             last_comment_page : 0,
//             last_update : Utc::now()
//         }
//     }
// }

impl DatabaseSchema for Site {

    fn get_table_name(

    ) -> String {
        "sites".to_string()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "id".to_string(),
            "name".to_string(),
            "actor_id".to_string(),
            "last_post_page".to_string(),
            "last_comment_page".to_string(),
            "last_update".to_string(),
        ]
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        HashMap::from([
            ("id".to_string(), DatabaseType::Uuid.not_null()),
            ("name".to_string(), DatabaseType::String(0).not_null()),
            ("actor_id".to_string(), DatabaseType::String(0).not_null().unique()),
            ("last_post_page".to_string(), DatabaseType::I32.not_null()),
            ("last_comment_page".to_string(), DatabaseType::I32.not_null()),
            ("last_update".to_string(), DatabaseType::DateTime.not_null())
        ])
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        // vec![
        //     &self.id,
        //     &self.name,
        //     &self.actor_id,
        //     &self.last_post_page,
        //     &self.last_comment_page,
        //     &self.last_update
        // ]
        unimplemented!("Insert not supported here.")
    }
}

impl PartialEq for Site {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Site {

}

impl Hash for Site {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
