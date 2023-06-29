use std::{hash::Hash, collections::HashMap};
use postgres::types::ToSql;
use crate::api::lemmy::models::post::PostData;
use super::{DatabaseSchema, DatabaseType};

impl DatabaseSchema for PostData {

    fn get_table_name(

    ) -> String {
        "posts".to_string()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "ap_id".to_string(),
            "url".to_string(),
            "name".to_string(),
            "body".to_string(),
            "score".to_string(),
            "author_actor_id".to_string(),
            "community_ap_id".to_string(),
        ]
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        HashMap::from([
            ("ap_id".to_string(), DatabaseType::Required(Box::new(DatabaseType::String(0)))),
            ("url".to_string(), DatabaseType::Optional(Box::new(DatabaseType::String(0)))),
            ("name".to_string(), DatabaseType::Required(Box::new(DatabaseType::String(0)))),
            ("body".to_string(), DatabaseType::Required(Box::new(DatabaseType::String(0)))),
            ("score".to_string(), DatabaseType::Required(Box::new(DatabaseType::String(0)))),
            ("author_actor_id".to_string(), DatabaseType::Required(Box::new(DatabaseType::String(0)))),
            ("community_ap_id".to_string(), DatabaseType::Required(Box::new(DatabaseType::String(0))))
        ])
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.post.ap_id,
            &self.post.url,
            &self.post.name,
            &self.post.body,
            &self.counts.score,
            &self.creator.actor_id,
            &self.community.actor_id
        ]
    }
}

impl PartialEq for PostData {
    fn eq(&self, other: &Self) -> bool {
        self.post.ap_id == other.post.ap_id
    }
}

impl Eq for PostData {

}

impl Hash for PostData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.post.ap_id.hash(state);
    }
}
