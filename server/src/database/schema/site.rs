use std::{
    collections::HashMap, 
    hash::Hash
};
use chrono::{
    DateTime, 
    Utc
};
use postgres::{
    types::ToSql, 
    Row
};
use uuid::Uuid;
use crate::{
    error::Result,
    api::lemmy::models::site::SiteView, 
    database::{
        dbo::get_database_client, 
        DatabasePool,
        DatabaseSchema,
    }
};
use super::DatabaseType;

#[derive(Clone)]
pub struct Site {
    pub id : Uuid,
    pub name : String,
    pub actor_id : String,
    pub software : String,
    pub last_post_page : i32,
    pub last_comment_page : i32,
    pub last_update : DateTime<Utc>
}

impl From<Site> for crate::api::lemmy::models::site::Site {
    fn from(value: Site) -> Self {
        Self {
            name : value.name,
            actor_id : value.actor_id
        }
    }
}

impl From<SiteView> for Site {
    fn from(site_view : SiteView) -> Self {
        Self {
            id : Uuid::new_v4(),
            name : site_view.site.name,
            actor_id : site_view.site.actor_id,
            software : "lemmy".to_string(),
            last_post_page : 0,
            last_comment_page : 0,
            last_update : Utc::now()
        }
    }
}

impl From<&Row> for Site {
    fn from(row : &Row) -> Self {
        Self {
            id : row.get("id"),
            name : row.get("name"),
            actor_id : row.get("actor_id"),
            software : row.get("software"),
            last_post_page : row.get("last_post_page"),
            last_comment_page : row.get("last_comment_page"),
            last_update : row.get("last_update"),
        }
    }
}

impl Site {

    pub async fn upsert(
        pool : DatabasePool,
        object : &Site,
    ) -> Result<bool> {

        let object = object.clone();

        get_database_client(&pool, move |client| {

            let value_columns = Site::get_column_names().into_iter().enumerate().map(|(i, _)| {
                format!("${}", i + 1)
            }).collect::<Vec<_>>()
                .join(", ");

            let name_index = Site::get_column_names()
                .iter()
                .position(|e| e == "name")
                .unwrap() + 1;

            let last_update_index = Site::get_column_names()
                .iter()
                .position(|e| e == "last_update")
                .unwrap() + 1;

            let query = format!("
                INSERT INTO {} ({})
                    VALUES ({})
                ON CONFLICT (actor_id)
                DO UPDATE SET \"name\" = ${}, last_update = ${}
            ",
                Site::get_table_name(),
                Site::get_column_names().join(", "),
                value_columns,
                name_index,
                last_update_index
            );

            client.execute(&query, &object.get_values())
                .map(|count| {
                    count == 1
                })
        }).await
    }
    
    pub async fn retrieve_all(
        pool : DatabasePool
    ) -> Result<Vec<Site>> {        

        get_database_client(&pool, move |client| {

            let query = format!("
                SELECT {} FROM {}
            ", Site::get_column_names().join(", "), Site::get_table_name());

            client.query(&query,
                &[] 
            ).map(|rows| {
                rows.iter().map(|row| {
                    Site::from(row)
                }).collect()
            })
        }).await
    }
}

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
            "software".to_string(),
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
            ("software".to_string(), DatabaseType::String(0).not_null()),
            ("last_post_page".to_string(), DatabaseType::I32.not_null()),
            ("last_comment_page".to_string(), DatabaseType::I32.not_null()),
            ("last_update".to_string(), DatabaseType::DateTime.not_null())
        ])
    }

    fn get_values(
        &self
    ) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.id,
            &self.name,
            &self.actor_id,
            &self.software,
            &self.last_post_page,
            &self.last_comment_page,
            &self.last_update
        ]
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
