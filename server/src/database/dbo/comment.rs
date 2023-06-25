use chrono::Utc;
use async_trait::async_trait;
use super::{
    DBO, 
    get_database_client, 
};
use crate::{
    error::LemmySearchError,
    database::DatabasePool,
    api::lemmy::models::{
        comment::{
            Comment, 
            CommentData, 
            Counts
        }, 
        post::{
            Post, 
            Creator
        }, 
        community::Community
    }    
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
    ) -> Result<(), LemmySearchError> {

        get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS comments (
                    ap_id             VARCHAR PRIMARY KEY,
                    content           VARCHAR NULL,
                    score             INTEGER,
                    author_actor_id   VARCHAR NOT NULL,
                    post_ap_id        VARCHAR NOT NULL,
                    community_ap_id   VARCHAR NOT NULL,
                    last_update       TIMESTAMP WITH TIME ZONE NOT NULL
                )
            ", &[]
            ).map(|_| {
                ()
            })
        })
    }

    async fn drop_table_if_exists(
        &self
    ) -> Result<(), LemmySearchError> {

        get_database_client(&self.pool, |client| {
            client.execute("DROP TABLE IF EXISTS comments", &[])
                .map(|_| {
                    ()
                })
        })
    }

    async fn retrieve(
        &self, 
        ap_id : &str
    ) -> Result<CommentData, LemmySearchError> {

        let ap_id = ap_id.to_owned();

        get_database_client(&self.pool, move |client| {
            client.query_one("
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
            ).map(|row| {
                CommentData { 
                    comment : Comment {
                        ap_id: ap_id.to_string(),
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
                        body : row.get("p.body"),
                        removed : Some(false),
                        deleted : Some(false),
                        language_id: 0
                    },
                    community : Community {
                        actor_id: row.get("c.ap_id"),
                        name: row.get("c.name"),
                        title: row.get("c.title")
                    }
                }
            })
        })
    }

    async fn upsert(
        &self,
        object : CommentData
    ) ->  Result<bool, LemmySearchError> {

        get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO comments (\"ap_id\", \"content\", \"score\", \"post_ap_id\", \"community_ap_id\", \"last_update\")
                    VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (ap_id)
                DO UPDATE SET \"content\" = $2, \"score\" = $3, \"post_ap_id\" = $4, \"community_ap_id\" = $5, \"last_update\" = $6
                ",
                    &[
                        &object.comment.ap_id,
                        &object.comment.content,
                        &object.counts.score,
                        &object.post.ap_id,
                        &object.community.actor_id,
                        &Utc::now()
                    ]
            ).map(|count| {
                count == 1
            })
        })
    }
}
