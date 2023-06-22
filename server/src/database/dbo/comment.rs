use chrono::Utc;
use async_trait::async_trait;
use crate::{
    database::DatabasePool,
    api::lemmy::models::{
        comment::{
            Comment, 
            CommentData, 
            Counts
        }, 
        post::{Post, Creator}, 
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
#[allow(unused_variables)]
impl DBO<CommentData> for CommentDBO {

    async fn create_table_if_not_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS comments (
                    ap_id             VARCHAR PRIMARY KEY,
                    content           VARCHAR NULL,
                    score             INTEGER,
                    author_actor_id   VARCHAR NOT NULL,
                    post_ap_id        VARCHAR NOT NULL,
                    community_ap_id   VARCHAR NOT NULL
                    late_update       DATE NOT NULL
                )
            ", &[]
            )
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn drop_table_if_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("DROP TABLE IF EXISTS comments", &[])
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
                INSERT INTO comments (ap_id, body, score, post_ap_id, community_ap_id, laste_updated) 
                    VALUES ($1, $2, $3, $4, $5, $6)
                ",
                    &[
                        &object.comment.ap_id,
                        &object.comment.content,
                        &object.counts.score,
                        &object.post.ap_id,
                        &object.community.actor_id,
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
        ap_id : &str
    ) -> Option<CommentData> {
        let ap_id = ap_id.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT m.body, 
                        m.score,
                        m.author_actor_id,
                        p.ap_id,
                        p.title, 
                        p.body,
                        c.ap_id,
                        c.name,
                        c.title
                    FROM comments AS m 
                        JOIN posts AS p ON p.ap_id = m.post_ap_id
                        JOIN community AS c ON c.ap_id = m.community_ap_id
                    WHERE m.ap_id = $1
                ",
                &[&ap_id] 
            ) {
                Ok(row) => Some(CommentData { 
                    comment : Comment {
                        ap_id: ap_id.clone(),
                        content: row.get(0),
                    },
                    counts: Counts {
                        score : Some(row.get(1))
                    },
                    creator : Creator {
                        actor_id : row.get(2)
                    },
                    post : Post {
                        ap_id: row.get(3),
                        name : row.get(4),
                        body : row.get(5)
                    },
                    community : Community {
                        actor_id: row.get(6),
                        name: row.get(7),
                        title: row.get(8)
                    }
                }),
                Err(_) => None
            }
        }).await.unwrap_or(None)
    }

    async fn update(
        &self, 
        ap_id : &str
    ) -> bool {
        false
    }

    async fn delete(
        &self, 
        ap_id : &str
    ) -> bool {
        false
    }
}
