use super::{
    lemmy::get_lemmy_actor_type, 
    kbin::get_kbin_actor_type, 
    mastodon::get_mastodon_actor_type
};

pub mod crawler;

pub enum ActorType {
    Post,
    Author,
    Community
}

pub fn get_actor_type(
    actor_id : &str
) -> Option<ActorType> {
    Some(get_lemmy_actor_type(actor_id)
            .or(get_kbin_actor_type(actor_id))
            .or(get_mastodon_actor_type(actor_id))
            .unwrap_or(ActorType::Post))
}
