use super::common::ActorType;

pub fn get_mastodon_actor_type(
    actor_id : &str
) -> Option<ActorType> {
    if actor_id.contains("/statuses/") {
        Some(ActorType::Post)
    } else if actor_id.contains("/users/") {
        Some(ActorType::Author)
    } else {
        None
    }
}