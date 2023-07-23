use std::{
    hash::Hash, 
    collections::HashMap
};
use postgres::types::ToSql;
use crate::{
    api::lemmy::models::id::LemmyId, 
    database::{
        dbo::get_database_client, 
        DatabasePool
    },
    error::Result
};
use super::{
    DatabaseSchema, 
    DatabaseType
};

impl LemmyId {
    pub async fn find(
        pool : DatabasePool,
        post_actor_id : &str,
        instance_actor_id : &str
    ) -> Result<i64> {

        let post_actor_id = post_actor_id.to_owned();
        let instance_actor_id = instance_actor_id.to_owned();

        Ok(get_database_client(&pool, move |client| {

            let query = format!("
                SELECT post_remote_id FROM {}
                    WHERE post_actor_id = $1
                        AND instance_actor_id = $2
            ", LemmyId::get_table_name());

            client.query_one(&query, &[&post_actor_id, &instance_actor_id])
                .map(|row| {
                    row.get(0)
                })
        }).await?)
    }
}

impl DatabaseSchema for LemmyId {

    fn get_table_name(

    ) -> String {
        "lemmy_ids".to_string()
    }

    fn get_keys(
    
    ) -> Vec<String> {
        vec![
            "post_actor_id".to_string(),
            "instance_actor_id".to_string() 
        ]    
    }

    fn get_column_names(
    
    ) -> Vec<String> {
        vec![
            "post_remote_id".to_string(),
            "post_actor_id".to_string(),
            "instance_actor_id".to_string()
        ]
    }

    fn get_column_types(
    
    ) -> HashMap<String, DatabaseType> {
        HashMap::from([
            ("post_remote_id".to_string(), DatabaseType::I64.not_null()),
            ("post_actor_id".to_string(), DatabaseType::String(0).not_null()),
            ("instance_actor_id".to_string(), DatabaseType::String(0).not_null())
        ])
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.post_remote_id,
            &self.post_actor_id,
            &self.instance_actor_id
        ]
    }
}

impl PartialEq for LemmyId {
    fn eq(&self, other: &Self) -> bool {
        self.post_actor_id == other.post_actor_id && self.instance_actor_id == other.instance_actor_id
    }
}

impl Eq for LemmyId {

}

impl Hash for LemmyId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.post_actor_id.hash(state);
        self.instance_actor_id.hash(state);
    }
}
