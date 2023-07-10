use std::{
    hash::Hash, 
    collections::HashMap
};
use postgres::types::ToSql;
use crate::api::lemmy::models::post::PostData;
use super::{
    DatabaseSchema, 
    DatabaseType
};

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
            "published".to_string(),
            "updated".to_string(),
            "nsfw".to_string(),
            "score".to_string(),
            "author_actor_id".to_string(),
            "community_ap_id".to_string(),
        ]
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        HashMap::from([
            ("ap_id".to_string(), DatabaseType::String(0).not_null()),
            ("url".to_string(), DatabaseType::String(0).nullable()),
            ("name".to_string(), DatabaseType::String(0).not_null()),
            ("body".to_string(), DatabaseType::String(0).nullable()),
            ("published".to_string(), DatabaseType::DateTime.not_null()),
            ("updated".to_string(), DatabaseType::DateTime.nullable()),
            ("nsfw".to_string(), DatabaseType::Bool.not_null()),
            ("score".to_string(), DatabaseType::I32.not_null()),
            ("author_actor_id".to_string(), DatabaseType::String(0).not_null()),
            ("community_ap_id".to_string(), DatabaseType::String(0).not_null())
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
            &self.post.published,
            &self.post.updated,
            &self.post.nsfw,
            &self.counts.score,
            &self.creator.actor_id,
            &self.community.actor_id,
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
