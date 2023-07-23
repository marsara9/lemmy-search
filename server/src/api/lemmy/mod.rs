use std::str::FromStr;
use actix_web::http::Uri;
use super::common::ActorType;

pub mod models;
pub mod crawler;

pub fn get_lemmy_actor_type(
    actor_id : &str
) -> Option<ActorType> {
    if actor_id.contains("/post/") {
        Some(ActorType::Post)
    } else if actor_id.contains("/u/") {
        Some(ActorType::Author)
    } else if actor_id.contains("/c/") {
        Some(ActorType::Community)
    } else {
        None
    }
}

pub fn build_lemmy_redirect_url(
    target_actor_id : &str,
    home_instance_actor_id : &str,
    path_part : &str
) -> crate::error::Result<String> {
    let actor_uri = Uri::from_str(&target_actor_id)?;
    let actor_name = actor_uri.path()
        .split("/")
        .collect::<Vec<_>>()
        .get(2)
        .unwrap()
        .to_owned();
    let actor_domain = actor_uri.host().unwrap();
    let home_uri = Uri::from_str(&home_instance_actor_id)?;
    let home_domain = home_uri.host()
        .unwrap();

    if actor_domain == home_domain {
        Ok(format!("https://{}/{}/{}", home_domain, path_part, actor_name))
    } else {
        Ok(format!("https://{}/{}/{}@{}", home_domain, path_part, actor_name, actor_domain))
    }
}
