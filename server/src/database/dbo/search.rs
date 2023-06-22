use crate::{
    api::{
        search::models::search::SearchPost, 
        lemmy::models::{
            comment::Comment, 
            post::Post
        }
    }, database::DatabasePool
};

use super::get_database_client;


pub struct SearchDatabase {
    pub pool : DatabasePool
}

impl SearchDatabase {

    pub fn new(pool : DatabasePool) -> Self {
        Self {
            pool
        }
    }

    pub async fn create_table_if_not_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS xref (
                    word_id         UUID NOT NULL,
                    post_ap_id      VARCHAR NOT NULL
                )
            ", &[]
            )
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub async fn drop_table_if_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("DROP TABLE IF EXISTS xref", &[])
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub async fn search(
        &self,
        query : &str,
        instance : &Option<String>,
        community : &Option<String>,
        author : &Option<String>
    ) -> Option<Vec<SearchPost>> {        
        let query = query.to_owned();
        let instance = instance.to_owned();
        let community = community.to_owned();
        let author = author.to_owned();
        match get_database_client(&self.pool, move|client| {
            
            let instance_query = match instance {
                Some(_) => "AND s.actor_id = #2",
                None => ""
            };
            let community_query = match instance {
                Some(_) => "AND c.ap_id = $3",
                None => ""
            };
            let author_query = match author {
                Some(_) => "AND p.author_actor_id = $4",
                None => ""
            };

            let query_string = format!("
                SELECT p.name, p.body FROM words AS w
                    JOIN posts AS p ON p.ap_ip = w.post_ap_id
                    JOIN communities AS c ON c.ap_id = p.community_ap_id
                WHERE w.word IN $1
                    {}
                    {}
                    {}
            ", instance_query, community_query, author_query);

            match client.query(&query_string, &[&query, &instance, &community, &author]) {
                Ok(rows) => {
                    rows.iter().map(|row| {
                        SearchPost {
                            name : row.get(0),
                            body : row.get(0),
                            score : 0,
                            comments : Vec::new()
                        }
                    }).collect()
                },
                Err(_) => Vec::<SearchPost>::new()
            }
        }).await {
            Ok(_) => None,
            Err(_) => None
        }
    }
}
