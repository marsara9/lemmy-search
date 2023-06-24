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

#[derive(Clone)]
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
                    url               VARCHAR NULL,
                    name              VARCHAR NOT NULL,
                    body              VARCHAR NULL,
                    score             INTEGER,
                    author_actor_id   VARCHAR NOT NULL,
                    community_ap_id   VARCHAR NOT NULL,
                    last_update       DATE
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
            client.execute("DROP TABLE IF EXISTS posts", &[])
                .unwrap_or_default()
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
                SELECT p.url,
                        p.name,
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
                        url : row.get("p.url"),
                        name: row.get("p.name"), 
                        body: row.get("p.body")
                    },
                    counts : Counts {
                        score : row.get("p.score"),
                        ..Default::default()
                    },
                    creator: Creator {
                        actor_id : row.get("p.author_actor_id")
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
        object : PostData
    ) -> bool {
        match get_database_client(&self.pool, move |client| {
            client.execute("
                INSERT INTO posts (ap_id, url, name, body, score, author_actor_id, community_ap_id, last_update) 
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (ap_id)
                DO UPDATE SET (url = $2, name = $3, body = $4, score = $5, author_actor_id = $6, community_ap_id = $7, last_update = $8)
                ",
                    &[
                        &object.post.ap_id,
                        &object.post.url,
                        &object.post.name,
                        &object.post.body,
                        &object.counts.score,
                        &object.creator.actor_id,
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
