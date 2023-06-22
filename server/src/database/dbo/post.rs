use chrono::Utc;
use async_trait::async_trait;
use crate::{
    database::DatabasePool,
    api::lemmy::models::{
        post::{
            Post, 
            PostData, Counts, Creator
        }, 
        community::Community
    }
};
use super::{
    DBO, 
    get_database_client
};

pub struct PostDBO {
    pool : DatabasePool
}

impl PostDBO {
    pub fn new(pool : DatabasePool) -> Self {
        return Self {
            pool
        }
    }
}

#[async_trait]
#[allow(unused_variables)]
impl DBO<PostData> for PostDBO {

    fn get_object_name(&self) -> &str {
        "PostData"
    }

    async fn create_table_if_not_exists(
        &self
    ) -> bool {
        match get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS posts (
                    ap_id             VARCHAR PRIMARY KEY,
                    name              VARCHAR(100) NOT NULL,
                    body              VARCHAR(300) NULL,
                    score             INTEGER,
                    author_actor_id   VARCHAR NOT NULL,
                    community_ap_id   VARCHAR NOT NULL,
                    last_update       DATE
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
            client.execute("DROP TABLE IF EXISTS posts", &[])
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }

    async fn retrieve(
        &self, 
        ap_id : &str
    ) -> Option<PostData> {
        let ap_id = ap_id.to_owned();
        get_database_client(&self.pool, move |client| {
            match client.query_one("
                SELECT p.name,
                        p.body,
                        p.score
                        p.author_actor_id,
                        c.ap_id,
                        c.name,
                        c.title,
                    FROM posts AS p
                        JOIN communities AS c on c.ap_id = m.community_id
                    WHERE p.ap_id = $1
                ",
                &[&ap_id] 
            ) {
                Ok(row) => Some(PostData { 
                    post: Post { 
                        ap_id: ap_id.clone(), 
                        name: row.get(0), 
                        body: row.get(1)
                    },
                    counts : Counts {
                        score : row.get(6),
                        ..Default::default()
                    },
                    creator: Creator {
                        actor_id : row.get(2)
                    },
                    community : Community { 
                        actor_id: row.get(3), 
                        name: row.get(4), 
                        title: row.get(5) 
                    }
                }),
                Err(_) => None
            }
        }).await.unwrap_or(None)
    }

    async fn upsert(
        &self,
        object : PostData
    ) -> bool {
        match get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO posts (ap_id, name, body, score, author_actor_id, community_ap_id, last_update) 
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (ap_id)
                DO UPDATE SET (name = $2, body = $3, score = $4, author_actor_id = $5, community_ap_id = $6, last_update = $7)
                ",
                    &[
                        &object.post.ap_id,
                        &object.post.name,
                        &object.post.body,
                        &object.counts.score,
                        &object.creator.actor_id,
                        &object.community.actor_id,
                        &Utc::now()
                    ]
            )
        }).await {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
