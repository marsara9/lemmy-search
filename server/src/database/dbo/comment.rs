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

#[derive(Clone)]
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

    fn get_object_name(&self) -> &str {
        "CommentData"
    }

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
                    community_ap_id   VARCHAR NOT NULL,
                    late_update       DATE NOT NULL
                )
            ", &[]
            ).unwrap_or_default()
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
                .unwrap_or_default()
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
                        p.url,
                        p.name, 
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
                        content: row.get("m.body"),
                    },
                    counts: Counts {
                        score : Some(row.get("m.score"))
                    },
                    creator : Creator {
                        actor_id : row.get("m.author_actor_id")
                    },
                    post : Post {
                        ap_id: row.get("p.ap_id"),
                        url : row.get("p.url"),
                        name : row.get("p.name"),
                        body : row.get("p.body")
                    },
                    community : Community {
                        actor_id: row.get("c.ap_id"),
                        name: row.get("c.name"),
                        title: row.get("c.title")
                    }
                }),
                Err(_) => None
            }
        }).await.unwrap_or(None)
    }

    async fn upsert(
        &self,
        object : CommentData
    ) -> bool {
        match get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO comments (ap_id, body, score, post_ap_id, community_ap_id, last_updated) 
                    VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (ap_id)
                DO UPDATE SET (body = $2, score = $3, post_ap_id = $4, community_ap_id = $5, last_updated = $6)
                ",
                    &[
                        &object.comment.ap_id,
                        &object.comment.content,
                        &object.counts.score,
                        &object.post.ap_id,
                        &object.community.actor_id,
                        &Utc::now()
                    ]
            ).unwrap_or_default()
        }).await {
            Ok(value) => value == 1,
            Err(_) => false
        }
    }
}
