use std::collections::HashSet;
use chrono::{
    DateTime, 
    Utc
};
use postgres::types::ToSql;

use super::get_database_client;
use crate::{
    error::Result,    
    database::DatabasePool,
    api::search::models::search::{
        SearchPost, 
        SearchAuthor, 
        SearchCommunity
    }
};

#[derive(Clone)]
pub struct SearchDatabase {
    pub pool : DatabasePool
}

impl SearchDatabase {

    const PAGE_LIMIT : i32 = 50;

    pub fn new(pool : DatabasePool) -> Self {
        Self {
            pool
        }
    }

    pub async fn search(
        &self,
        query : &HashSet<String>,
        instance : &Option<String>,
        community : &Option<String>,
        author : &Option<String>,
        nsfw : &Option<bool>,
        since: &Option<DateTime<Utc>>,
        until: &Option<DateTime<Utc>>,
        home_instance : &str,
        page : i32
    ) -> Result<(Vec<SearchPost>, i32)> {        

        let query = query.to_owned();
        let instance = instance.to_owned();
        let community = community.to_owned();
        let author = author.to_owned();
        let nsfw = nsfw.to_owned();
        let since = since.to_owned();
        let until = until.to_owned();
        let home_instance = home_instance.to_owned();

        get_database_client(&self.pool, move |client| {

            let query_terms = Vec::<String>::from_iter(query.into_iter());

            let instance_query = match instance {
                Some(_) => "AND c.ap_id LIKE $2 || '%'",
                None => "AND $2 = $2"
            };
            let community_query = match community {
                Some(_) => "AND c.ap_id = $3",
                None => "AND $3 = $3"
            };
            let author_query = match author {
                Some(_) => "AND p.author_actor_id = $4",
                None => "AND $4 = $4"
            };
            let nsfw_query: &str = match nsfw {
                Some(_) => "AND p.nsfw = $5",
                None => "AND $5 = TRUE"
            };
            let since_query: &str = match since {
                Some(_) => "AND p.updated > $6",
                None => "AND $6 = $6"
            };
            let until_query: &str = match until {
                Some(_) => "AND p.updated < $7",
                None => "AND $7 = $7"
            };

            let instance = instance.unwrap_or("".to_string());
            let community = community.unwrap_or("".to_string());
            let author = author.unwrap_or("".to_string());
            let nsfw = nsfw.unwrap_or(true);
            let since = since.unwrap_or(Utc::now());
            let until = until.unwrap_or(Utc::now());

            // Finds all words that match the search criteria, then filter those results
            // by any additional criteria that the user may have, such as instance, 
            // community, or author.  Next, count the number of matches each post has
            // and sort first by the number of matches and then if there's a conflict
            // by the total number of upvotes that the post has.
            let query_string = format!("
                SELECT
                    p.url as p_url,
                    p.name as p_name,
                    left(p.body, 300) as p_body,
                    p.updated as p_updated,
                    
                    l.post_remote_id as p_remote_id,
                    
                    a.ap_id as a_actor_id,
                    a.avatar as a_avatar,
                    a.name as a_name,
                    a.display_name as a_display_name,
                    
                    c.ap_id as c_actor_id,
                    c.icon as c_icon,
                    c.name as c_name,
                    c.title as c_title,

                    COUNT(*) OVER() AS total_results
                    FROM (
                        SELECT COUNT(p.ap_id) AS matches, 
                                p.ap_id, 
                                p.url, 
                                p.name, 
                                p.body, 
                                p.author_actor_id, 
                                p.community_ap_id, 
                                p.score, 
                                p.nsfw, 
                                p.updated
                            FROM xref AS x
                                INNER JOIN words AS w ON w.id = x.word_id
                                INNER JOIN posts AS p ON p.ap_id = x.post_ap_id
                            WHERE w.word = any($1)
                            GROUP BY p.ap_id
                    ) AS p
                INNER JOIN authors AS a ON a.ap_id = p.author_actor_id
                INNER JOIN communities AS c ON c.ap_id = p.community_ap_id
                INNER JOIN lemmy_ids AS l ON l.post_actor_id = p.ap_id
                WHERE l.instance_actor_id = $8
                    {instance_query}
                    {community_query}
                    {author_query}
                    {nsfw_query}
                    {since_query}
                    {until_query}
                ORDER BY
                    matches DESC,
                    p.score DESC
                LIMIT {}
                OFFSET $9
            ", Self::PAGE_LIMIT);

            let mut total_results = 0;

            let offset = (Self::PAGE_LIMIT * (page - 1)) as i64;

            let params : Vec<&(dyn ToSql + Sync)> = vec![
                &query_terms,   // $1
                &instance,      // $2
                &community,     // $3
                &author,        // $4
                &nsfw,          // $5
                &since,         // $6
                &until,         // $7
                &home_instance, // $8
                &offset         // $9
            ];

            let results = client.query(
                &query_string, 
                &params
            ).map(|rows| {
                rows.iter().map(|row| {
                    let temp : i64 = row.get("total_results");
                    total_results = temp as i32;

                    SearchPost {
                        url : row.get("p_url"),
                        name : row.get("p_name"),
                        body : row.get("p_body"),
                        updated: row.get("p_updated"),
                        remote_id : row.get("p_remote_id"),
                        author : SearchAuthor {
                            actor_id: row.get("a_actor_id"),
                            avatar : row.get("a_avatar"),
                            name : row.get("a_name"),
                            display_name : row.get("a_display_name"),
                        },
                        community : SearchCommunity {
                            actor_id : row.get("c_actor_id"),
                            icon : row.get("c_icon"),
                            name : row.get("c_name"),
                            title : row.get("c_title")
                        }
                    }
                }).collect()
            })?;

            Ok((results, total_results))
        }).await
    }
}
