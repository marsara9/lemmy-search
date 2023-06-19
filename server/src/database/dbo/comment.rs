use chrono::Utc;
use uuid::Uuid;
use async_trait::async_trait;
use crate::{
    database::DatabasePool,
    api::lemmy::models::{
        comment::{
            Comment, 
            CommentData, 
            Counts
        }, 
        post::Post, 
        community::Community
    }
};
use super::{
    DBO, 
    get_database_client
};

pub struct CommentDBO {
    pool : DatabasePool
}

impl CommentDBO {
    pub fn new(pool : DatabasePool) -> Self {
        return Self {
            pool
        }
    }
}

#[async_trait]
impl DBO<CommentData> for CommentDBO {

    async fn create_table_if_not_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS comments (
                    remote_id         INT,
                    instance_actor_id VARCHAR NOT NULL,     
                    body              VARCHAR NULL,
                    upvotes           INTEGER,
                    late_update       DATE
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
        object : &CommentData
    ) -> bool {        
        let object = object.to_owned();
        match get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO comments (id, post_id, body, upvotes, laste_updated) 
                    VALUES ($1, $2, $3)",
                    &[
                        &Uuid::new_v4(),
                        &Uuid::new_v4(),
                        &object.comment.content,
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
        uuid : &Uuid
    ) -> Option<CommentData> {
        let uuid = uuid.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT m.body, 
                        m.upvotes, 
                        p.title, 
                        p.body,
                        c.name,
                        c.title
                    FROM comments AS m WHERE id = $1
                    JOIN posts AS p on p.id = m.post_id
                    JOIN communities AS c on c.id = m.community_id
                ",
                &[&uuid] 
            ) {
                Ok(row) => Some(CommentData { 
                    comment : Comment {
                        content: row.get(0),
                    },
                    counts: Counts {
                        score : Some(row.get(1))
                    },
                    post : Post {
                        id: 0,
                        name : row.get(2),
                        body : row.get(3)
                    },
                    community : Community {
                        id: 0,
                        name: row.get(4),
                        title: row.get(5)
                    }
                }),
                Err(_) => None
            }
        }).await.unwrap_or(None)
    }

    async fn update(
        &self, 
        uuid : &Uuid
    ) -> bool {
        false
    }

    async fn delete(
        &self, 
        uuid : &Uuid
    ) -> bool {
        false
    }
}
