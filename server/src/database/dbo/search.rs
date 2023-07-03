use std::collections::HashSet;
use super::{
    get_database_client
};
use crate::{
    error::Result,    
    database::DatabasePool,
    api::{
        search::models::search::{
            SearchPost, 
            SearchAuthor, 
            SearchCommunity
        }
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
        preferred_instance : &str,
        page : i32
    ) -> Result<(Vec<SearchPost>, i32)> {        

        let query = query.to_owned();
        let instance = instance.to_owned();
        let community = community.to_owned();
        let author = author.to_owned();
        let preferred_instance = preferred_instance.to_owned();

        get_database_client(&self.pool, move |client| {

            let temp = Vec::<String>::from_iter(query.into_iter());

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

            let instance = instance.unwrap_or("".to_string());
            let community = community.unwrap_or("".to_string());
            let author = author.unwrap_or("".to_string());

            // Finds all words that match the search criteria, then filter those results
            // by any additional criteria that the user may have, such as instance, 
            // community, or author.  Next, count the number of matches each post has
            // and sort first by the number of matches and then if there's a conflict
            // by the total number of upvotes that the post has.
            let query_string = format!("
            SELECT
                    p.url,
                    p.name,
                    p.body,
                    
                    l.post_remote_id,
                    
                    a.ap_id,
                    a.avatar,
                    a.name,
                    a.display_name,
                    
                    c.ap_id,
                    c.icon,
                    c.name,
                    c.title,

                    COUNT(*) OVER() AS total_results
                    FROM (
                        SELECT COUNT(p.ap_id) AS matches, p.ap_id, p.url, p.name, p.body, p.author_actor_id, p.community_ap_id, p.score
                            FROM xref AS x
                                INNER JOIN words AS w ON w.id = x.word_id
                                INNER JOIN posts AS p ON p.ap_id = x.post_ap_id
                            WHERE w.word = any($1)
                            GROUP BY p.ap_id
                    ) AS p
                INNER JOIN authors AS a ON a.ap_id = p.author_actor_id
                INNER JOIN communities AS c ON c.ap_id = p.community_ap_id
                INNER JOIN lemmy_ids AS l ON l.post_actor_id = p.ap_id
                WHERE l.instance_actor_id = $5
                    {}
                    {}
                    {}
                ORDER BY
                    matches DESC,
                    p.score DESC
                LIMIT {}
                OFFSET $6
            ", instance_query, community_query, author_query, Self::PAGE_LIMIT);

            let mut total_results = 0;

            let offset = (Self::PAGE_LIMIT * (page - 1)) as i64;

            let results = client.query(&query_string, &[&temp, &instance, &community, &author, &preferred_instance, &offset])
                .map(|rows| {
                    rows.iter().map(|row| {
                        let temp : i64 = row.get(12);
                        total_results = temp as i32;

                        SearchPost {
                            url : row.get(0),
                            name : row.get(1),
                            body : row.get(2),
                            remote_id : row.get(3),
                            author : SearchAuthor {
                                actor_id: row.get(4),
                                avatar : row.get(5),
                                name : row.get(6),
                                display_name : row.get(7),
                            },
                            community : SearchCommunity {
                                actor_id : row.get(8),
                                icon : row.get(9),
                                name : row.get(10),
                                title : row.get(11)
                            }
                        }
                    }).collect()
                })?;

            Ok((results, total_results))
        }).await
    }
}
