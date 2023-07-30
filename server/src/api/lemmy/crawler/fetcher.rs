use std::fmt::Debug;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;
use robotstxt::DefaultMatcher;
use serde_json::{
    Map, 
    Value
};
use crate::{
    error::{
        Result, 
        LemmySearchError
    },
    api::lemmy::models::{
        common::{
            ListingType, 
            SortType
        }, 
        site::{
            SiteResponse, 
            SiteRequest,
            FederatedInstancesResponse,
            FederatedInstancesRequest
        },
        post::{
            PostData, 
            PostListRequest, 
            PostListResponse, 
        }, comment::{
            CommentListRequest, 
            CommentListResponse, 
            CommentData
        }
    }
};
use serde::{
    Serialize, 
    de::DeserializeOwned
};

lazy_static! {
    static ref INSTANCE : Regex = Regex::new(r"https://(?P<domain>.+)/").unwrap();
}

#[derive(Clone)]
pub struct Fetcher {
    instance : String,
    client : Client
}

impl Fetcher {

    pub const DEFAULT_LIMIT : i32 = 50;

    pub fn new(
        client : Client,
        instance : String
    ) -> Self {
        let instance = match INSTANCE.captures(&instance) {
            Some(caps) => {
                caps["domain"].to_owned()
            },
            None => {
                instance
            }
        };

        Self {
            client,
            instance
        }
    }

    fn get_url(
        &self,
        path : &str
    ) -> String {
        return format!("https://{}{}", self.instance, path);
    }

    pub async fn fetch_if_can_crawl(
        &self,
        user_agent : &str
    ) -> Result<bool> {

        let url = self.get_url("/robots.txt");

        println!("Connecting to {}...", url);
    
        let robots_txt = self.client
            .get(url)
            .send()
            .await?
            .text()
            .await?;

        Ok(DefaultMatcher::default().one_agent_allowed_by_robots(&robots_txt, user_agent, "/"))
    }

    pub async fn fetch_site_data(
        &self
    ) -> Result<SiteResponse> {
        let params = SiteRequest;
        let url = self.get_url("/api/v3/site");
        self.fetch_json::<SiteRequest, SiteResponse>(&url, params)
            .await
    }

    pub async fn fetch_instances(
        &self
    ) -> Result<FederatedInstancesResponse> {
        let params = FederatedInstancesRequest;
        let url = self.get_url("/api/v3/federated_instances");
        self.fetch_json(&url, params)
            .await
    }

    pub async fn fetch_posts(
        &self,
        page : i32
    ) -> Result<Vec<PostData>> {
        let params = PostListRequest {
            type_: Some(ListingType::Local),
            sort: Some(SortType::Old),
            limit: Self::DEFAULT_LIMIT,
            page: page,
            ..Default::default()
        };

        let url = self.get_url("/api/v3/post/list");

        self.fetch_json(&url, params)
            .await
            .map(|view: PostListResponse| {
                view.posts
            })
    }

    #[allow(unused_variables)]
    pub async fn fetch_comments(
        &self,
        remote_post_id : i64,
        page : i32
    ) -> Result<Vec<CommentData>> {
        // let params = CommentListRequest {
        //     post_id: Some(remote_post_id),
        //     limit: Self::DEFAULT_LIMIT,
        //     page,
        //     ..Default::default()
        // };

        // let url = self.get_url("/api/v3/comment/list");

        // self.fetch_json(&url, params)
        //     .await
        //     .map(|view: CommentListResponse| {
        //         view.comments
        //     })

        Ok(Vec::new())
    }

    pub async fn get_internal_id(
        &self,
        actor_id : &str   
    ) -> Result<Option<i64>> {

        let url = self.get_url("/api/v3/resolve_object");

        #[derive(Serialize, Debug)]
        struct ResolveObjectRequest {
            q : String
        }

        let json = self.fetch_json(&url, ResolveObjectRequest {
            q : actor_id.to_string()
        }).await
            .map(|result : serde_json::Value| {
                result.as_object()
                    .map(|m| m.to_owned())
                    .ok_or(LemmySearchError::JsonError)
            })??;

        if json.contains_key("error") {
            Ok(None)
        } else if json.contains_key("post") {
            Ok(self.get_actor_internal_id(json, "post"))
        } else if json.contains_key("comment") {
            Ok(self.get_actor_internal_id(json, "comment"))
        } else if json.contains_key("person") {
            Ok(self.get_actor_internal_id(json, "person"))
        } else if json.contains_key("community") {
            Ok(self.get_actor_internal_id(json, "community"))
        } else {
            Err(LemmySearchError::JsonError)
        }
    }

    fn get_actor_internal_id(
        &self,
        json : Map<String, Value>,
        type_ : &str
    ) -> Option<i64> {
        json.get(type_)
            .map(|v| {
                v.get(type_)
            }).flatten()
            .map(|v| {
                v.get("id")
            }).flatten()
            .map(|v| {
                v.as_i64()
            }).flatten()
    }

    async fn fetch_json<T, R>(
        &self,
        url : &str,
        params : T
    ) -> Result<R>
    where
        T : Serialize + Sized + Debug,
        R : Default + DeserializeOwned
    {
        println!("Connecting to {}...", url);
        println!("\twith params {:?}...", params);
    
        let result = self.client
            .get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(result)
    }
}
