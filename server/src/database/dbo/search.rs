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
        query : &String,
        instance : &Option<String>,
        community : &Option<String>,
        author : &Option<String>,
        nsfw : &bool,
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

            let instance_query = match instance {
                Some(_) => "AND LOWER(c.ap_id) LIKE $2 || '%'",
                None => "AND $2 = $2"
            };
            let community_query = match community {
                Some(_) => "AND LOWER(c.ap_id) = $3",
                None => "AND $3 = $3"
            };
            let author_query = match author {
                Some(_) => "AND LOWER(p.author_actor_id) = $4",
                None => "AND $4 = $4"
            };
            let nsfw_query: &str = if nsfw {
                ""
            } else {
                "AND p.nsfw = FALSE"
            };            
            let since_query: &str = match since {
                Some(_) => "AND p.updated > $5",
                None => "AND $5::TIMESTAMPTZ = $5::TIMESTAMPTZ"
            };
            let until_query: &str = match until {
                Some(_) => "AND p.updated < $6",
                None => "AND $6::TIMESTAMPTZ = $6::TIMESTAMPTZ"
            };

            let instance = instance.unwrap_or("".to_string());
            let community = community.unwrap_or("".to_string());
            let author = author.unwrap_or("".to_string());
            let since = since.unwrap_or(Utc::now());
            let until = until.unwrap_or(Utc::now());

            let query_string = format!("
            SELECT 
                    p.ap_id as p_actor_id,
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

                    COUNT(*) OVER() AS total_results,
                    ts_rank_cd(p.com_search, websearch_to_tsquery($1), 12) AS rank 
                FROM posts AS p
                    INNER JOIN authors AS a ON a.ap_id = p.author_actor_id
                    INNER JOIN communities AS c ON c.ap_id = p.community_ap_id
                    INNER JOIN lemmy_ids AS l ON l.post_actor_id = p.ap_id
                    WHERE p.com_search @@ websearch_to_tsquery($1)
                        AND l.instance_actor_id = $7
                        {instance_query}
                        {community_query}
                        {author_query}
                        {nsfw_query}
                        {since_query}
                        {until_query}
                ORDER BY
                    rank DESC,
                    p.score DESC
                LIMIT {}
                OFFSET $8
            ", Self::PAGE_LIMIT);

            let mut total_results = 0;

            let offset = (Self::PAGE_LIMIT * (page - 1)) as i64;

            let params : Vec<&(dyn ToSql + Sync)> = vec![
                &query,         // $1
                &instance,      // $2
                &community,     // $3
                &author,        // $4
                &since,         // $5
                &until,         // $6
                &home_instance, // $7
                &offset         // $8
            ];

            let results = client.query(
                &query_string, 
                &params
            ).map(|rows| {
                rows.iter().map(|row| {
                    let temp : i64 = row.get("total_results");
                    total_results = temp as i32;

                    SearchPost {
                        actor_id : row.get("p_actor_id"),                  
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
                            title : row.get("c_title"),
                            number_of_matches : None
                        }
                    }
                }).collect()
            })?;

            Ok((results, total_results))
        }).await
    }

    pub async fn find_community(
        &self,
        query : &String,
        instance : &Option<String>,
        author : &Option<String>,
        nsfw : &bool,
        since: &Option<DateTime<Utc>>,
        until: &Option<DateTime<Utc>>,
        home_instance : &str,
        page : i32
    ) -> Result<(Vec<SearchCommunity>, i32)> {

        let query = query.to_owned();
        let instance = instance.to_owned();
        let author = author.to_owned();
        let nsfw = nsfw.to_owned();
        let since = since.to_owned();
        let until = until.to_owned();
        let home_instance = home_instance.to_owned();

        get_database_client(&self.pool, move |client| {

            let instance_query = match instance {
                Some(_) => "AND c.ap_id LIKE $2 || '%'",
                None => "AND $2 = $2"
            };
            let author_query = match author {
                Some(_) => "AND p.author_actor_id = $3",
                None => "AND $3 = $3"
            };
            let nsfw_query: &str = if nsfw {
                ""
            } else {
                "AND p.nsfw = FALSE"
            };            
            let since_query: &str = match since {
                Some(_) => "AND p.updated > $4",
                None => "AND $4::TIMESTAMPTZ = $4::TIMESTAMPTZ"
            };
            let until_query: &str = match until {
                Some(_) => "AND p.updated < $5",
                None => "AND $5::TIMESTAMPTZ = $5::TIMESTAMPTZ"
            };

            let instance = instance.unwrap_or("".to_string());
            let author = author.unwrap_or("".to_string());
            let since = since.unwrap_or(Utc::now());
            let until = until.unwrap_or(Utc::now());

            let query_string = format!("
                SELECT 
                        c.ap_id AS c_actor_id, 
                        c.icon AS c_icon, 
                        c.name AS c_name, 
                        c.title AS c_title, 
                        COUNT(p.*) AS matches,
                        COUNT(*) OVER() AS total_results
                    FROM posts AS p
                        INNER JOIN communities AS c ON c.ap_id = p.community_ap_id
                        INNER JOIN lemmy_ids AS l ON l.post_actor_id = p.ap_id
                    WHERE p.com_search @@ websearch_to_tsquery($1)
                        AND l.instance_actor_id = $6
                        {instance_query}
                        {author_query}
                        {nsfw_query}
                        {since_query}
                        {until_query}
                    GROUP BY c.ap_id
                    ORDER BY 
                        matches DESC
                    LIMIT {}
                    OFFSET $7
            ", Self::PAGE_LIMIT);

            let offset = (Self::PAGE_LIMIT * (page - 1)) as i64;

            let params : Vec<&(dyn ToSql + Sync)> = vec![
                    &query,         // $1
                    &instance,      // $2
                    &author,        // $3
                    &since,         // $4
                    &until,         // $5
                    &home_instance, // $6
                    &offset         // $7
                ];

            let mut total_results = 0;

            let results = client.query(
                &query_string, 
                &params
            ).map(|rows| {
                rows.iter().map(|row| {
                    let temp : i64 = row.get("total_results");
                    total_results = temp as i32;

                    SearchCommunity {
                        actor_id : row.get("c_actor_id"),
                        icon : row.get("c_icon"),
                        name : row.get("c_name"),
                        title : row.get("c_title"),
                        number_of_matches : Some(row.get("matches"))
                    }
                }).collect()
            })?;

            Ok((results, total_results))
        }).await
    }
}
