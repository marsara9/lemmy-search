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
#[allow(unused_variables)]
impl DBO<CommentData> for CommentDBO {

    async fn create_table_if_not_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS comments (
                    remote_id         INTEGER,
                    instance          VARCHAR NOT NULL,
                    content           VARCHAR NULL,
                    score             INTEGER,
                    late_update       DATE,
                    PRIMARY KEY(remote_id, instance)
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
        instance : &str,
        object : &CommentData
    ) -> bool {
        let instance = instance.to_owned();  
        let object = object.to_owned();
        match get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO comments (remote_id, instance, post_id, body, upvotes, laste_updated) 
                    VALUES ($1, $2, $3)
                ",
                    &[
                        &object.comment.id,
                        &instance,
                        &object.post.id,
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
        remote_id : &i64,
        instance : &str
    ) -> Option<CommentData> {
        let remote_id = remote_id.to_owned();
        let instance = instance.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT m.body, 
                        m.upvotes,
                        p.remote_id,
                        p.title, 
                        p.body,
                        c.remote_id,
                        c.name,
                        c.title
                    FROM comments AS m 
                        JOIN posts AS p on p.id = m.post_id
                        OIN communities AS c on c.id = m.community_id
                    WHERE m.remote_id = $1 AND m.instance = $2
                ",
                &[&remote_id, &instance] 
            ) {
                Ok(row) => Some(CommentData { 
                    comment : Comment {
                        id: remote_id.clone(),
                        content: row.get(0),
                    },
                    counts: Counts {
                        score : Some(row.get(1))
                    },
                    post : Post {
                        id: row.get(2),
                        name : row.get(3),
                        body : row.get(4)
                    },
                    community : Community {
                        id: row.get(5),
                        name: row.get(6),
                        title: row.get(7)
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
