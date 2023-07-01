use std::{
    collections::HashSet, 
    fmt::Debug,
};
use deadpool::managed::Object;
use deadpool_r2d2::Manager;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;

use crate::{
    database::{
        DatabasePool, 
        schema::{
            DatabaseSchema, 
            word::Word, 
            xref::Search
        }
    }, 
    error::Result,
    api::lemmy::models::{
        post::PostData, 
        id::LemmyId, author::Author, community::Community,
    }, 
    crawler::analyzer::Analyzer
};

pub struct CrawlerDatabase {
    client : Object<Manager<PostgresConnectionManager<NoTls>>>
}

impl CrawlerDatabase {

    pub async fn init(pool : DatabasePool) -> Result<Self> {
        let client = pool.get().await?;

        Ok(Self {
            client
        })
    }

    pub async fn bulk_update_post(
        &mut self,
        instance_actor_id : &str,
        posts : &Vec<PostData>
    ) -> Result<()> {

        let mut authors = HashSet::<_>::new();
        let mut communities = HashSet::<_>::new();
        let mut lemmy_ids = HashSet::<_>::new();
        let mut all_words = HashSet::<Word>::new();
        let mut xrefs = HashSet::<_>::new();

        for post in posts {
            authors.insert(post.creator.clone());
            communities.insert(post.community.clone());
            lemmy_ids.insert(LemmyId {
                post_remote_id : post.post.id.clone(),
                post_actor_id : post.post.ap_id.clone(),
                instance_actor_id : instance_actor_id.to_string()
            });
            let words = post.post.get_distinct_words().into_iter().map(|word| {
                Word::from(word)
            }).collect::<HashSet<_>>();
            all_words.extend(words);
        }

        self.update_authors(&authors).await?;
        self.update_communities(&communities).await?;

        let words = all_words.into_iter().collect();
        let posts2 = posts.into_iter().map(|p| {
            p.clone()
        }).collect();
        
        self.update_words(&words).await?;
        self.update_posts(&posts2).await?;

        for post in posts {
            xrefs.extend(self.get_xrefs_for_post(post).await?);
        }

        self.update_lemmy_ids(&lemmy_ids).await?;
        self.update_xref(&xrefs).await?;

        if xrefs.len() == 0 && words.len() != 0 {
            println!("WARNING NO xrefs were calculated for posts!.")
        }

        Ok(())
    }

    pub async fn get_xrefs_for_post(
        &mut self,
        post_data : &PostData
    ) -> Result<HashSet<Search>> {

        let words = post_data.post.get_distinct_words()
            .into_iter()
            .collect::<Vec<_>>();

        let query = "
            SELECT w.id, p.ap_id FROM posts AS p
            JOIN words AS w ON w.id = w.id
            WHERE w.word = any($1)
                AND p.ap_id = $2
        ".to_string();

        let post_data = post_data.clone();

        Ok(self.client.interact(move |client| {
            client.query(&query, &[&words, &post_data.post.ap_id])
                .map(|rows| {
                    rows.into_iter().map(|row| {
                        Search {
                            word_id : row.get(0),
                            post_ap_id : row.get(1)
                        }
                    }).collect::<HashSet<_>>()
                })
        }).await??)
    }

    pub async fn bulk_update_lemmy_ids(
        &mut self,
        instance_actor_id : &str,
        posts : &Vec<PostData>
    ) -> Result<u64> {

        let mut lemmy_ids = HashSet::<_>::new();

        for post in posts {
            lemmy_ids.insert(LemmyId {
                post_remote_id : post.post.id.clone(),
                post_actor_id : post.post.ap_id.clone(),
                instance_actor_id : instance_actor_id.to_string()
            });
        }

        self.update_lemmy_ids(&lemmy_ids)
            .await
    }

    fn bulk_get_query<T : DatabaseSchema + Debug + Clone>(
        objects : &HashSet<T>
    ) -> Option<String> {
        let objects = objects.clone();

        let mut values = Vec::<String>::new();
        let mut index = 1;
        for item in &objects {
            let t = item.get_values().into_iter().enumerate().map(|(i, _)| {
                format!("${}", index + i)
            }).collect::<Vec<_>>();
            values.push(format!("({})", t.join(", ")));
            index += t.len();
        }

        let exclude = T::get_column_names()
            .into_iter()
            .filter(|column| {
                !T::get_keys().contains(column)
            })
            .map(|column| {
                format!("{} = excluded.{}", column, column)
            })
            .collect::<Vec<_>>()
            .join(",\n\t\t\t");

        if values.is_empty() {
            // Nothing to insert; skip
            return None
        }

        let query = if exclude.is_empty() {
            format!("
                INSERT INTO {} ({})
                    VALUES 
                        {}
                ON CONFLICT ({}) 
                    DO NOTHING
            ", 
                T::get_table_name(),
                T::get_column_names().join(", "),
                values.join(",\n\t\t\t\t"),
                T::get_keys().join(", ")
            )
        } else if T::get_keys().is_empty() {
            format!("
                INSERT INTO {} ({})
                    VALUES 
                        {}
            ", 
                T::get_table_name(),
                T::get_column_names().join(", "),
                values.join(",\n\t\t\t\t")
            )
        } else { 
            format!("
                INSERT INTO {} ({})
                    VALUES 
                        {}
                ON CONFLICT ({}) DO
                    UPDATE SET
                        {}
            ", 
                T::get_table_name(),
                T::get_column_names().join(", "),
                values.join(",\n\t\t\t\t"),
                T::get_keys().join(", "),
                exclude
            )
        };

        Some(query)
    }

    async fn update_authors(
        &mut self,
        objects : &HashSet<Author>
    ) -> Result<u64> {
        let objects = objects.clone();
        
        Ok(self.client.interact(move |client| {
            let q = Self::bulk_get_query(&objects);

            let params = objects.get_values();

            match q {
                Some(query) => {
                    client.execute(&query, &params)
                },
                None => Ok(0)
            }
        }).await??)
    }

    async fn update_communities(
        &mut self,        
        objects : &HashSet<Community>
    ) -> Result<u64> {
        let objects = objects.clone();
        
        Ok(self.client.interact(move |client| {
            let q = Self::bulk_get_query(&objects);

            let params = objects.get_values();

            match q {
                Some(query) => {
                    client.execute(&query, &params)
                },
                None => Ok(0)
            }
        }).await??)
    }

    async fn update_posts(
        &mut self,
        objects : &HashSet<PostData>
    ) -> Result<u64> {
        let objects = objects.clone();
        
        Ok(self.client.interact(move |client| {
            let q = Self::bulk_get_query(&objects);

            let params = objects.get_values();

            match q {
                Some(query) => {
                    client.execute(&query, &params)
                },
                None => Ok(0)
            }
        }).await??)
    }

    async fn update_lemmy_ids(
        &mut self,
        objects : &HashSet<LemmyId>
    ) -> Result<u64> {
        let objects = objects.clone();
        
        Ok(self.client.interact(move |client| {
            let q = Self::bulk_get_query(&objects);

            let params = objects.get_values();

            match q {
                Some(query) => {
                    client.execute(&query, &params)
                },
                None => Ok(0)
            }
        }).await??)
    }

    async fn update_xref(
        &mut self,
        objects : &HashSet<Search>
    ) -> Result<u64> {
        let objects = objects.clone();
        
        Ok(self.client.interact(move |client| {
            let q = Self::bulk_get_query(&objects);

            let params = objects.get_values();

            match q {
                Some(query) => {
                    client.execute(&query, &params)
                },
                None => Ok(0)
            }
        }).await??)
    }

    async fn update_words(
        &mut self,
        objects : &HashSet<Word>
    ) -> Result<u64> {
        let objects = objects.clone();
        
        Ok(self.client.interact(move |client| {

            let params = objects.get_values();

            let mut values = Vec::<String>::new();
            let mut index = 1;
            for item in &objects {
                let t = item.get_values().into_iter().enumerate().map(|(i, _)| {
                    format!("${}", index + i)
                }).collect::<Vec<_>>();
                values.push(format!("({})", t.join(", ")));
                index += t.len();
            }

            let query = format!("
                INSERT INTO {} ({})
                    VALUES 
                        {}
                ON CONFLICT (word) 
                    DO NOTHING
            ", 
                Word::get_table_name(),
                Word::get_column_names().join(", "),
                values.join(",\n\t\t\t\t")
            );

            client.execute(&query, &params)
        }).await??)
    }

}