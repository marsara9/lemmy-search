use std::{
    collections::HashSet, 
    fmt::Debug
};
use crate::{
    database::{
        DatabaseClient, 
        DatabasePool, schema::{DatabaseSchema, word::Word, xref::Search}
    }, 
    error::Result,
    api::lemmy::models::{
        post::PostData, 
        id::LemmyId,
    }, 
    crawler::analyzer::Analyzer
};

pub struct CrawlerDatabase {
    client : DatabaseClient
}

impl CrawlerDatabase {

    pub fn init(pool : DatabasePool) -> Result<Self> {
        let client = pool.get()?;

        Ok(Self {
            client
        })
    }

    pub fn bulk_update_post(
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
            let xref = words.iter().map(|word| {
                Search {
                    word_id: word.id.clone(),
                    post_ap_id: post.post.ap_id.clone()
                }
            }).collect::<Vec<_>>();
            xrefs.extend(xref);

            all_words.extend(words);
        }

        let words = all_words.into_iter().collect();
        let posts = posts.into_iter().map(|p| {
            p.clone()
        }).collect();

        self.bulk_update(&authors)?;
        self.bulk_update(&communities)?;
        self.bulk_update(&posts)?;
        self.bulk_update(&lemmy_ids)?;
        self.bulk_update_words(&words)?;
        self.bulk_update(&xrefs)?;

        Ok(())
    }

    pub fn bulk_update_lemmy_ids(
        &mut self,
        instance_actor_id : &str,
        posts : &Vec<PostData>
    ) -> Result<()> {

        let mut lemmy_ids = HashSet::<_>::new();

        for post in posts {
            lemmy_ids.insert(LemmyId {
                post_remote_id : post.post.id.clone(),
                post_actor_id : post.post.ap_id.clone(),
                instance_actor_id : instance_actor_id.to_string()
            });
        }

        self.bulk_update(&lemmy_ids)?;

        Ok(())
    }

    fn bulk_update<T : DatabaseSchema + Debug>(
        &mut self,
        objects : &HashSet<T>
    ) -> Result<()> {
        let params = objects.get_values();

        let mut values = Vec::<String>::new();
        let mut index = 1;
        for item in objects {
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

        self.client.execute(&query, &params)?;

        Ok(())
    }

    fn bulk_update_words(
        &mut self,
        objects : &HashSet<Word>
    ) -> Result<()> {
        let params = objects.get_values();

        let mut values = Vec::<String>::new();
        let mut index = 1;
        for item in objects {
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

        self.client.execute(&query, &params)?;

        Ok(())
    }

}