pub mod models;

use std::{
    collections::HashMap, 
    sync::Mutex
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
        dbo::site::SiteDBO, 
        DatabasePool
    }
};

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
        _pool : Data<Mutex<DatabasePool>>,
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
