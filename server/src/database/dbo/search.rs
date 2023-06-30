use std::collections::HashSet;
use super::{
    get_database_client
};
use crate::{
    error::LemmySearchError,
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
    ) -> Result<Vec<SearchPost>, LemmySearchError> {        

        let query = query.to_owned();
        let instance = instance.to_owned();
        let community = community.to_owned();
        let author = author.to_owned();
        let preferred_instance = preferred_instance.to_owned();

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

            // Finds all words that match the search criteria, then filter those results
            // by any additional criteria that the user may have, such as instance, 
            // community, or author.  Next, count the number of matches each post has
            // and sort first by the number of matches and then if there's a conflict
            // by the total number of upvotes that the post has.
            let query_string = format!("
                SELECT p.url, p.name, p.body, l.post_remote_id, a.avatar, a.name, a.display_name, c.icon, c.name, c.title FROM (
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
                    INNER JOIN lemmy_ids AS l ON l.post_actor_id = p.ap_id
                WHERE l.instance_actor_id = $5
                ORDER BY 
                    matches DESC,
                    p.score DESC
            ", instance_query, community_query, author_query);

            client.query(&query_string, &[&temp, &instance, &community, &author, &preferred_instance])
                .map(|rows| {
                    rows.iter().map(|row| {
                        SearchPost {
                            url : row.get(0),
                            name : row.get(1),
                            body : row.get(2),
                            remote_id : row.get(3),
                            author : SearchAuthor {
                                avatar : row.get(4),
                                name : row.get(5),
                                display_name : row.get(6),
                            },
                            community : SearchCommunity {
                                icon : row.get(7),
                                name : row.get(8),
                                title : row.get(9)
                            }
                        }
                    }).collect()
                })
        })
    }
}
