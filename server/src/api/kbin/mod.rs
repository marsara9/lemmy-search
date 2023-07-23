pub mod models;

use super::common::ActorType;

pub fn get_kbin_actor_type(
    actor_id : &str
) -> Option<ActorType> {
    if actor_id.contains("/p/") {
        Some(ActorType::Post)
    } else if actor_id.contains("/u/") {
        Some(ActorType::Author)
    } else if actor_id.contains("/m/") {
        Some(ActorType::Community)
    } else {
        None
    }
}
