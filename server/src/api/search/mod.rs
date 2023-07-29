pub mod models;
pub mod filters;

use self::models::{
    search::Version, 
    redirect::Redirect
};
use super::{
    lemmy::{
        crawler::LemmyCrawler, 
        models::id::LemmyId,
        build_lemmy_redirect_url
    }, 
    common::ActorType
};
use std::{
    collections::{
        HashMap, 
        HashSet
    }, 
    time::Instant
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
use reqwest::StatusCode;
use crate::{
    error::{
        LogError, 
        LemmySearchError
    },
    api::{
        search::{
            models::search::{
                SearchQuery,
                SearchResult, 
                FindCommunityResult
            }, 
            filters::{
                instance::InstanceFilter, 
                community::CommunityFilter, 
                author::AuthorFilter, 
                nsfw::NSFWFilter, 
                date::DateFilter
            }
        }, 
        common::get_actor_type
    }, 
    database::{
        dbo::search::SearchDatabase,
        Context, 
        schema::site::Site
    },
    config::Config
};

pub struct SearchHandler {
    pub routes : HashMap<String, Route>
}

impl SearchHandler {

    const PAGE_LIMIT : usize = 50;

    pub fn new(config : &Config) -> Self {
        let mut routes = HashMap::<String, Route>::new();
        if config.development_mode {
            routes.insert("/heartbeat".to_string(), get().to(Self::heartbeat));
            routes.insert("/crawl".to_string(), get().to(Self::crawl));
        }
        routes.insert("/api/version".to_string(), get().to(Self::version));
        routes.insert("/api/donate".to_string(), get().to(Self::donate));
        routes.insert("/api/search/posts".to_string(), get().to(Self::search_posts));
        routes.insert("/api/search/communities".to_string(), get().to(Self::search_communities));
        routes.insert("/api/instances".to_string(), get().to(Self::get_instances));
        routes.insert("/api/redirect".to_string(), get().to(Self::redirect));

        Self {
            routes
        }
    }

    pub async fn version<'a>(

    ) -> Result<impl Responder> {
        Ok(
            Json(
                Version {
                    version: env!("CARGO_PKG_VERSION").to_string()
                }
            ).customize()
                .insert_header(("cache-control", "public, max-age=86400"))
        )
    }

    pub async fn donate<'a>(
        context : Data<Context>
    ) -> Result<impl Responder> {
        let donations = context.config.clone().donations;
        Ok(
            Json(donations)
                .customize()
                .insert_header(("cache-control", "public, max-age=86400"))
        )
    }

    /**
     * This method solely exists as just a way to confirm that the server is responding.
     * It should never do anything besides just respond with 'Ready'.
     */
    pub async fn heartbeat<'a>(
        
    ) -> Result<impl Responder> {
        Ok(
            "Ready"
                .customize()
                .insert_header(("cache-control", "no-store"))
        )
    }

    /**
     * Temporary endpoint to allow for more easily testing the crawler.
     */
    pub async fn crawl<'a>(
        context : Data<Context>
    ) -> Result<impl Responder> {

        let config = context.config.clone();

        tokio::spawn(async move {
            let crawler = LemmyCrawler::new(
                config.crawler.seed_instance.clone(), 
                (*context.into_inner()).clone()
            ).unwrap();

            let _ = crawler.crawl()
                .await
                .log_error("The manually triggered crawler encountered an error.", true);
        });

        Ok(
            "Started"
                .customize()
                .insert_header(("cache-control", "no-store"))
        )
    }

    /**
     * This is the actual search function that is called when the user enters a query.
     * 
     * This method will tokenize the query string and extract any filters provided by
     * the user before sending that information off to the Database to query.
     */
    pub async fn search_posts<'a>(
        context : Data<Context>,
        search_query: Query<SearchQuery>
    ) -> Result<impl Responder> {

        let start = Instant::now();

        println!("Searching...");

        let query = search_query.query.to_owned();
        let mut modified_query = query.clone();

        let instance = modified_query.get_instance_filter();
        let community = modified_query.get_community_filter();
        let author = modified_query.get_author_filter();
        let nsfw = modified_query.get_nsfw_filter();
        let since = modified_query.get_since_filter();
        let until = modified_query.get_until_filter();

        // normalize the query string to lowercase.
        modified_query = modified_query.to_lowercase()
            .trim()
            .to_string();

        // Log search query
        println!("\tfor '{}'", modified_query);

        // The preferred instance is sent without the https://, re-add it back.
        let home_instance_actor_id = format!("https://{}/", search_query.home_instance);

        let page = search_query.page.unwrap_or(1).max(1);

        println!("\tpage: {}", page);

        let search = SearchDatabase::new(context.pool.clone());
        let search_results = search.search(
            &modified_query, 
            &instance, 
            &community, 
            &author, 
            &nsfw,
            &since,
            &until,
            &home_instance_actor_id,
            page
        ).await
            .log_error("Error during search.", true)
            .map_err(|err| {
                actix_web::error::ErrorInternalServerError(err)
            })?;

        let len = search_results.1;
        let total_pages = (len as f32 / Self::PAGE_LIMIT as f32).ceil() as i32;

        // tokenize the search query, remove any non-alphanumeric characters from the string
        // and remove any words that are less than 3 characters long.
        let query_terms = modified_query.replace(|c : char| {
            !c.is_alphanumeric() && !c.is_whitespace()
        }, " ")
            .split_whitespace()
            .map(|word| {
                word.trim().to_string()
            }).filter(|word| {
                word.len() > 2
            }).collect::<HashSet<String>>();

        // Capture the duration that the search took so we can report it back
        // to the user.
        let duration = start.elapsed();

        let results: SearchResult = SearchResult {
            original_query_terms : query_terms,
            posts : search_results.0,
            total_results : len,
            total_pages : total_pages,
            time_taken: duration
        };

        Ok(
            Json(results)
                .customize()
                .insert_header(("cache-control", "public, max-age=86400"))
        )
    }

    pub async fn search_communities<'a>(
        context : Data<Context>,
        search_query: Query<SearchQuery>
    ) -> Result<impl Responder> {

        let start = Instant::now();

        println!("Finding community...");

        let query = search_query.query.to_owned();
        let mut modified_query = query.clone();

        let instance = modified_query.get_instance_filter();
        let author = modified_query.get_author_filter();
        let nsfw = modified_query.get_nsfw_filter();
        let since = modified_query.get_since_filter();
        let until = modified_query.get_until_filter();

        // normalize the query string to lowercase.
        modified_query = modified_query.to_lowercase()
            .trim()
            .to_string();

        // Log search query
        println!("\tfor '{}'", modified_query);

        // The preferred instance is sent without the https://, re-add it back.
        let home_instance_actor_id = format!("https://{}/", search_query.home_instance);

        let page = search_query.page.unwrap_or(1).max(1);

        println!("\tpage: {}", page);

        let search = SearchDatabase::new(context.pool.clone());
        let search_results = search.find_community(
            &modified_query, 
            &instance, 
            &author, 
            &nsfw,
            &since,
            &until,
            &home_instance_actor_id,
            page
        ).await
            .log_error("Error during search.", true)
            .map_err(|err| {
                actix_web::error::ErrorInternalServerError(err)
            })?;

        let len = search_results.1;
        let total_pages = (len as f32 / Self::PAGE_LIMIT as f32).ceil() as i32;

        // tokenize the search query, remove any non-alphanumeric characters from the string
        // and remove any words that are less than 3 characters long.
        let query_terms = modified_query.replace(|c : char| {
            !c.is_alphanumeric() && !c.is_whitespace()
        }, " ")
            .split_whitespace()
            .map(|word| {
                word.trim().to_string()
            }).filter(|word| {
                word.len() > 2
            }).collect::<HashSet<String>>();

        // Capture the duration that the search took so we can report it back
        // to the user.
        let duration = start.elapsed();

        let results: FindCommunityResult = FindCommunityResult {
            original_query_terms : query_terms,
            communities : search_results.0,
            total_results : len,
            total_pages : total_pages,
            time_taken: duration
        };

        Ok(
            Json(results)
                .customize()
                .insert_header(("cache-control", "public, max-age=86400"))
        )
    }

    /**
     * Returns a list of all available instances that this search engine has seen.
     * 
     * These will be ultimately used as the 'preferred instance' when calling
     * the actual search method.
     */
    pub async fn get_instances<'a>(
        context : Data<Context>
    ) -> Result<impl Responder> {
        let sites = Site::retrieve_all(context.pool.clone())
            .await.map_err(|err| {
                actix_web::error::ErrorInternalServerError(err)
            }).map(|sites| {
                sites.into_iter().map(|site| {
                    crate::api::lemmy::models::site::Site::from(site)
                }).collect::<Vec<_>>()
            })?;

        Ok(
            Json(sites)
                .customize()
                .insert_header(("cache-control", "public, max-age=86400"))
        )
    }

    pub async fn redirect<'a>(
        context : Data<Context>,
        source : Query<Redirect>
    ) -> Result<impl Responder> {

        println!("Redirecting to: {}", source.actor_id);

        match get_actor_type(&source.actor_id) {
            Some(actor_type) => match actor_type {
                ActorType::Post => {
                    let internal_id = LemmyId::find(
                        context.pool.clone(), 
                        &source.actor_id,
                        &source.home_instance
                    ).await.map_err(|err| {
                        actix_web::error::ErrorInternalServerError(err)
                    })?;
            
                    let location = format!("{}post/{}", source.home_instance, internal_id);
                    
                    Ok(""
                        .customize()
                        .append_header(("location", location))
                        .with_status(StatusCode::SEE_OTHER)
                    )
                },
                ActorType::Author => {
                    let location = build_lemmy_redirect_url(
                        &source.actor_id, 
                        &source.home_instance, 
                        "u"
                    ).map_err(|err| {
                        actix_web::error::ErrorInternalServerError(err)
                    })?;
    
                    Ok(""
                        .customize()
                        .append_header(("location", location))
                        .with_status(StatusCode::SEE_OTHER)
                    )
                },
                ActorType::Community => {
                    let location = build_lemmy_redirect_url(
                        &source.actor_id, 
                        &source.home_instance, 
                        "c"
                    ).map_err(|err| {
                        actix_web::error::ErrorInternalServerError(err)
                    })?;
    
                    Ok(""
                        .customize()
                        .append_header(("location", location))
                        .with_status(StatusCode::SEE_OTHER)
                    )
                }
            },
            None => {
                Err(actix_web::error::ErrorNotAcceptable(LemmySearchError::Generic("Invalid actor_id URL.")))
            }
        }
    }
}
