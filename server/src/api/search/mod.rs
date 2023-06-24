pub mod models;

use regex::Regex;
use lazy_static::lazy_static;
use self::models::search::SearchPost;
use std::{
    collections::HashMap, 
    sync::Mutex, time::Instant
};
use actix_web::{
    web::{
        Json, 
        Query,
        Data,
        get
    }, 
    Responder, 
    Result, 
    Route
};
use crate::{
    api::search::models::search::{
        SearchQuery,
        SearchResult
    }, 
    database::{
        dbo::{
            site::SiteDBO, 
            search::SearchDatabase
        }, 
        DatabasePool
    }
};

lazy_static! {
    static ref SITE_MATCH : Regex = Regex::new(r" site:(?P<site>https://[\w\-\.]+)").unwrap();
    static ref COMMUNITY_MATCH : Regex = Regex::new(r" community:(?P<community>\w+@[\w\-\.]+)").unwrap();
    static ref AUTHOR_MATCH : Regex = Regex::new(r" author:(?P<author>\w+@[\w\-\.]+)").unwrap();
}

pub struct SearchHandler {
    pub routes : HashMap<String, Route>
}

impl SearchHandler {

    pub fn new() -> Self {
        let mut routes = HashMap::<String, Route>::new();
        routes.insert("/heartbeat".to_string(), get().to(Self::heartbeat));
        routes.insert("/search".to_string(), get().to(Self::search));
        routes.insert("/instances".to_string(), get().to(Self::get_instances));

        Self {
            routes
        }
    }

    pub async fn heartbeat<'a>(

    ) -> Result<impl Responder> {
        Ok("Ready")
    }

    pub async fn search<'a>(
        pool : Data<Mutex<DatabasePool>>,
        search_query: Query<SearchQuery>
    ) -> Result<impl Responder> {

        let start = Instant::now();

        let query = search_query.query.to_owned();
        let mut modified_query = query.clone();
        let instance = match SITE_MATCH.captures(&query) {
            Some(caps) => {
                let cap = &caps["site"];
                modified_query = modified_query.replace(cap, "")
                    .replace("site:", "");
                Some(cap.to_string())
            },
            None => None
        };
        let community = match COMMUNITY_MATCH.captures(&query) {
            Some(caps) => {
                let cap = &caps["community"];
                modified_query = modified_query.replace(cap, "")
                    .replace("community:", "");
                Some(cap.to_string())
            },
            None => None
        };
        let author = match AUTHOR_MATCH.captures(&query) {
            Some(caps) => {
                let cap = &caps["author"];
                modified_query = modified_query.replace(cap, "")
                    .replace("author:", "");
                Some(cap.to_string())
            },
            None => None
        };

        println!("Searching for '{}'", modified_query);
        match &instance {
            Some(value) => {
                println!("\tSite: '{}'", value);
            },
            None => {}
        }
        match &community {
            Some(value) => {
                println!("\tCommunity: '{}'", value);
            },
            None => {}
        }
        match &author {
            Some(value) => {
                println!("\tAuthor: '{}'", value);
            },
            None => {}
        }

        let search = SearchDatabase::new(pool.lock().unwrap().clone());
        let search_results = search.search(&modified_query, &instance, &community, &author)
            .await;

        let duration = start.elapsed();

        let results: SearchResult = SearchResult {
            original_query : search_query.into_inner(),
            search_results : match search_results {
                Some(value) => value,
                None => Vec::<SearchPost>::new(),
            },
            total_pages : 0,
            time_taken: duration
        };
        Ok(Json(results))
    }

    pub async fn get_instances<'a>(
        pool : Data<Mutex<DatabasePool>>
    ) -> Result<impl Responder> {
        let pool = pool.lock().unwrap();
        Ok(Json(
            SiteDBO::new(pool.clone())
                .retrieve_all()
                .await
        )) 
    }
}
