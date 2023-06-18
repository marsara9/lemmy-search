use std::collections::HashSet;

use super::Crawler;
use crate::api::lemmy::models::{
    post::Post, 
    comment::Comment
};

trait Analyizer {
    fn get_distinct_words_in_post(
        post : Post
    ) -> HashSet<String>;

    fn get_distinct_words_in_comment(
        comment : Comment
    ) -> HashSet<String>;
}

impl Analyizer for Crawler {

    fn get_distinct_words_in_post(
        post : Post
    ) -> HashSet<String> {
        let mut words = HashSet::<String>::new();
        for word in post.name.split_whitespace() {
            words.insert(word.to_string());
        }
        match post.body {
            Some(body) => for word in body.split_whitespace() {
                words.insert(word.to_string());
            },
            None => {}
        }
        words
    }

    fn get_distinct_words_in_comment(
        comment : Comment
    ) -> HashSet<String> {
        HashSet::from_iter(comment.content.split_whitespace().map(|word|
            word.to_string()
        ))
    }
}
