pub mod models;

use std::collections::HashMap;

use actix_web::{
    get, 
    web::{
        Json, 
        Query, self
    }, 
    Responder, 
    Result, Route
};
use crate::{
    api::search::models::search::{
        SearchQuery,
        SearchResult
    }
};

pub struct SearchHandler {
    pub routes : HashMap<String, Route>
}

impl SearchHandler {
    pub fn new(
    ) -> Self {
        let mut routes = HashMap::<String, Route>::new();
        routes.insert("/heartbeat".to_string(), web::get().to(Self::heartbeat));
        routes.insert("/search".to_string(), web::get().to(Self::search));
        routes.insert("/instances".to_string(), web::get().to(Self::get_instances));

        SearchHandler {
            routes
         }
    }

    pub async fn heartbeat<'a>(

    ) -> Result<impl Responder> {
        Ok("Ready")
    }

    pub async fn search<'a>(
        search_query: Query<SearchQuery>
    ) -> Result<impl Responder> {
        let search_results = SearchResult {
            original_query : search_query.into_inner(),
            search_results : Vec::new(),
            total_pages : 0
    
        };
        Ok(Json(search_results))
    }

    pub async fn get_instances<'a>(

    ) -> Result<impl Responder> {
        let instances = Vec::<String>::new();
    
        Ok(Json(instances))   
    }
    
}

#[get("/heartbeat")]
pub async fn heartbeat(

) -> Result<impl Responder> {
    Ok("Ready")
}

#[get("/search")]
pub async fn search(
    search_query: Query<SearchQuery>
) -> Result<impl Responder> {
    let search_results = SearchResult {
        original_query : search_query.into_inner(),
        search_results : Vec::new(),
        total_pages : 0

    };
    Ok(Json(search_results))
}

#[get("/instances")]
pub async fn get_instances(

) -> Result<impl Responder> {
    let instances = Vec::<String>::new();

    Ok(Json(instances))   
}
