use std::collections::HashSet;

use crate::api::lemmy::models::{
    post::Post, 
    comment::Comment
};

pub trait Analyzer {
    fn get_distinct_words(
        &self
    ) -> HashSet<String>;
}

impl Analyzer for Post {

    fn get_distinct_words(
        &self
    ) -> HashSet<String> {
        let mut words = HashSet::<String>::new();
        let name_trimed = self.name.replace(|c : char| {
            !c.is_ascii_alphanumeric() && !c.is_whitespace()
        }, " ").to_lowercase();
        for word in name_trimed.split_whitespace() {
            words.insert(word.to_lowercase().trim().to_string());
        }
        match &self.body {
            Some(body) => {
                let body_trimed = body.replace(|c : char| {
                    !c.is_ascii_alphanumeric() && !c.is_whitespace()
                }, " ").to_lowercase();
                for word in body_trimed.split_whitespace() {
                    words.insert(word.to_lowercase().trim().to_string());
                }
            },
            None => {}
        }
        words
    }
}

impl Analyzer for Comment {

    fn get_distinct_words(
        &self,
    ) -> HashSet<String> {
        HashSet::from_iter(self.content.replace(|c : char| {
            !c.is_ascii_alphanumeric() && !c.is_whitespace()
        }, " ").to_lowercase().split_whitespace().map(|word|
            word.to_lowercase().trim().to_string()
        ))
    }
}
