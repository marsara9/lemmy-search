use chrono::Utc;
use async_trait::async_trait;
use crate::{
    database::DatabasePool,
    api::lemmy::models::{
        post::{
            Post, 
            PostData, Counts
        }, 
        community::Community
    }
};
use super::{
    DBO, 
    get_database_client
};

pub struct PostDAO {
    pool : DatabasePool
}

impl PostDAO {
    pub fn new(pool : DatabasePool) -> Self {
        return Self {
            pool
        }
    }
}

#[async_trait]
#[allow(unused_variables)]
impl DBO<PostData> for PostDAO {

    async fn create_table_if_not_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS  (
                    remote_id         INT,
                    instance          VARCHAR NOT NULL,
                    name              VARCHAR NOT NULL,
                    body              VARCHAR NULL,
                    score             INTEGER,
                    last_update       DATE
                )
            ", &[]
            )
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn create(
        &self, 
        instance : &str,
        object : &PostData
    ) -> bool {        
        let instance = instance.to_owned();  
        let object = object.to_owned();
        match get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO posts (remote_id, instance, name, body, score, last_update) 
                    VALUES ($1, $2, $3)",
                    &[
                        &object.post.id,
                        &instance,
                        &object.post.name,
                        &object.post.body,
                        &object.counts.score,
                        &Utc::now()
                    ]
            )
        }).await {
            Ok(_) => true,
            Err(_) => false
        } 
    }

    async fn retrieve(
        &self, 
        remote_id : &i64,
        instance : &str
    ) -> Option<PostData> {
        let remote_id = remote_id.to_owned();
        let instance = instance.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT p.name,
                        p.body,
                        c.remote_id,
                        c.name,
                        c.title,
                        p.score
                    FROM posts AS p
                        JOIN communities AS c on c.id = m.community_id
                    WHERE p.remote_id = $1 AND p.instance = $2
                ",
                &[&remote_id, &instance] 
            ) {
                Ok(row) => Some(PostData { 
                    post: Post { 
                        id: remote_id.clone(), 
                        name: row.get(0), 
                        body: row.get(1)
                    },
                    community : Community { 
                        id: row.get(2), 
                        name: row.get(3), 
                        title: row.get(4) 
                    },
                    counts : Counts {
                        score : row.get(5),
                        ..Default::default()
                    }
                }),
                Err(_) => None
            }
        }).await.unwrap_or(None)
    }

    async fn update(
        &self, 
        remote_id : &i64,
        instance : &str
    ) -> bool {
        false
    }

    async fn delete(
        &self, 
        remote_id : &i64,
        instance : &str
    ) -> bool {
        false
    }
}
