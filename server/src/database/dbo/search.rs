use std::collections::HashSet;
use postgres::types::ToSql;
use uuid::Uuid;
use super::get_database_client;
use crate::{
    error::LemmySearchError,
    database::DatabasePool,
    api::{
        search::models::search::{SearchPost, SearchAuthor, SearchCommunity}, 
        lemmy::models::{
            post::Post, 
            comment::Comment
        },
    }
};

#[derive(Clone)]
pub struct SearchDatabase {
    pub pool : DatabasePool
}

impl SearchDatabase {

    pub fn new(pool : DatabasePool) -> Self {
        Self {
            pool
        }
    }

    pub async fn create_table_if_not_exists(
        &self
    ) -> Result<(), LemmySearchError> {
        get_database_client(&self.pool, |client| {
            client.execute("
                CREATE TABLE IF NOT EXISTS xref (
                    word_id         UUID NOT NULL,
                    post_ap_id      VARCHAR NOT NULL
                )
            ", &[]
            ).map(|_| {
                ()
            })
        })
    }

    #[allow(unused)]
    pub async fn drop_table_if_exists(
        &self
    ) -> Result<(), LemmySearchError> {
        get_database_client(&self.pool, |client| {
            client.execute("DROP TABLE IF EXISTS xref", &[])
                .map(|_| {
                    ()
                })
        })
    }

    pub async fn upsert_post(
        &self,
        words : HashSet<String>,
        post : Post
    ) -> Result<(), LemmySearchError> {

        get_database_client(&self.pool, move |client| {

            let mut transaction = client.transaction()?;
            let deleted = transaction.execute("DELETE FROM xref WHERE post_ap_id = $1", &[&post.ap_id])?;

            let words = words.into_iter().collect::<Vec<String>>();
            let rows = transaction.query("SELECT id FROM words WHERE word = any($1)", &[&words])?;
            let ids = rows.into_iter().map(|row| {
                row.get::<&str, Uuid>("id")
            }).collect::<Vec<Uuid>>();

            let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
            for id in &ids {
                params.push(id);
            }

            let mut query = format!("INSERT INTO xref (word_id, post_ap_id) VALUES ");
            if ids.len() != 0 {
                for index in 0..ids.len() {
                    query += format!("(${} , $1),", index+2).as_str();
                }
                query = query.trim_end_matches(",").to_string();
                params.insert(0, &post.ap_id);
                transaction.execute(&query, &params)?;
            } else {
                println!("WARNING: post was inserted but had 0 words? {} associations were deleted however.", deleted);
                println!("{:#?}", post);
            }

            transaction.commit()
        })
    }

    pub async fn upsert_comment(
        &self,
        words : HashSet<String>,
        comment : Comment
    ) -> Result<(), LemmySearchError> {
        // TODO
        Ok(())
    }

    pub async fn search(
        &self,
        query : &HashSet<String>,
        instance : &Option<String>,
        community : &Option<String>,
        author : &Option<String>
    ) -> Result<Vec<SearchPost>, LemmySearchError> {        

        let query = query.to_owned();
        let instance = instance.to_owned();
        let community = community.to_owned();
        let author = author.to_owned();

        get_database_client(&self.pool, move |client| {

            let temp = Vec::<String>::from_iter(query.into_iter());

            let instance_query = match instance {
                Some(_) => "AND s.actor_id = $2",
                None => ""
            };
            let community_query = match community {
                Some(_) => "AND c.ap_id = $3",
                None => ""
            };
            let author_query = match author {
                Some(_) => "AND p.author_actor_id = $4",
                None => ""
            };

            let instance = instance.unwrap_or("".to_string());
            let community = community.unwrap_or("".to_string());
            let author = author.unwrap_or("".to_string());

            // Finds all words that match the search critera, then filter those results
            // by any additional critera that the user may have, such as instance, 
            // community, or author.  Next, count the number of matches each post has
            // and sort first by the number of matches and then if there's a conflict
            // by the total number of upvotes that the post has.
            let query_string = format!("
                SELECT p.url, p.name, p.body, a.avatar, a.name, a.display_name, c.icon, c.name, c.title FROM (
                    SELECT COUNT (p.ap_id) as matches, p.ap_id FROM xref AS x
                        LEFT JOIN words AS w ON w.id = x.word_id 
                        LEFT JOIN posts AS p ON p.ap_id = x.post_ap_id
                        LEFT JOIN communities AS c ON c.ap_id = p.community_ap_id
                        LEFT JOIN sites AS s ON c.ap_id LIKE s.actor_id || '%'
                    WHERE w.word = any($1)
                        AND $2 = $2
                        AND $3 = $3
                        AND $4 = $4
                        {}
                        {}
                        {}
                    GROUP BY p.ap_id
                ) AS t
                    INNER JOIN posts AS p ON p.ap_id = t.ap_id
                    INNER JOIN communities AS c ON c.ap_id = p.community_ap_id
                    INNER JOIN authors AS a ON a.ap_id = p.author_actor_id
                ORDER BY 
                    matches DESC,
                    p.score DESC
            ", instance_query, community_query, author_query);

            client.query(&query_string, &[&temp, &instance, &community, &author])
                .map(|rows| {
                    rows.iter().map(|row| {
                        SearchPost {
                            url : row.get(0),
                            name : row.get(1),
                            body : row.get(2),
                            remote_id : "".to_string(), // TODO
                            author : SearchAuthor {
                                avatar : row.get(3),
                                name : row.get(4),
                                display_name : row.get(5),
                            },
                            community : SearchCommunity {
                                icon : row.get(6),
                                name : row.get(7),
                                title : row.get(8)
                            }
                        }
                    }).collect()
                })
        })
    }
}
