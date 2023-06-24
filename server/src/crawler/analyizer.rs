use std::collections::HashSet;

use crate::api::lemmy::models::{
    post::Post, 
    comment::Comment
};

pub struct Analyizer;

impl Analyizer {

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_distinct_words_in_post(
        &self,
        post : &Post
    ) -> HashSet<String> {
        let mut words = HashSet::<String>::new();
        for word in post.name.split_whitespace() {
            words.insert(word.to_lowercase().to_string());
        }
        match &post.body {
            Some(body) => for word in body.split_whitespace() {
                words.insert(word.to_lowercase().to_string());
            },
            None => {}
        }
        words
    }

    pub fn get_distinct_words_in_comment(
        &self,
        comment : &Comment
    ) -> HashSet<String> {
        HashSet::from_iter(comment.content.split_whitespace().map(|word|
            word.to_lowercase().to_string()
        ))
    }
}
