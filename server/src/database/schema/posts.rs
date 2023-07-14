use std::{
    hash::Hash, 
    collections::HashMap
};
use chrono::{
    DateTime, 
    Utc
};
use postgres::types::ToSql;
use crate::api::lemmy::models::post::PostData;
use super::{
    DatabaseSchema, 
    DatabaseType, 
    author::Author, 
    community::Community
};

#[derive(Debug, Clone)]
pub struct Post {
    pub ap_id : String,
    pub name : String,
    pub body : Option<String>,
    pub updated : DateTime<Utc>,
    pub nsfw : bool,
    pub score : i32,
    pub author : Author,
    pub community : Community,
}

impl From<&PostData> for Post {
    fn from(
        post_data : &PostData
    ) -> Self {
        Self {
            ap_id : post_data.post.ap_id.clone(),
            name : post_data.post.name.clone(),
            body : post_data.post.body.clone(),
            updated : post_data.post.updated.unwrap_or(post_data.post.published).and_utc(),
            nsfw : post_data.post.nsfw.unwrap_or(false),
            score : post_data.counts.score.clone(),
            author: Author::from(&post_data.creator),
            community: Community::from(&post_data.community)
        }
    }
}

impl DatabaseSchema for Post {

    fn get_table_name(

    ) -> String {
        "posts".to_string()
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "ap_id".to_string(),
            "name".to_string(),
            "body".to_string(),
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
            ("name".to_string(), DatabaseType::String(0).not_null()),
            ("body".to_string(), DatabaseType::String(0).nullable()),
            ("updated".to_string(), DatabaseType::DateTime.not_null()),
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
            &self.ap_id,
            &self.name,
            &self.body,
            &self.updated,
            &self.nsfw,
            &self.score,
            &self.author.actor_id,
            &self.community.actor_id,
        ]
    }
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.ap_id == other.ap_id
    }
}

impl Eq for Post {

}

impl Hash for Post {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ap_id.hash(state);
    }
}
