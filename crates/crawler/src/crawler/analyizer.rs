use std::collections::HashSet;

use super::Crawler;
use lemmy_search_common::{
    models::{
        LemmyPost, 
        LemmyComment
    }
};

trait Analyizer {
    fn get_distinct_words_in_post(
        post : LemmyPost
    ) -> HashSet<String>;

    fn get_distinct_words_in_comment(
        comment : LemmyComment
    ) -> HashSet<String>;
}

impl Analyizer for Crawler {

    fn get_distinct_words_in_post(
        post : LemmyPost
    ) -> HashSet<String> {
        let mut words = HashSet::<String>::new();
        for word in post.title.split_whitespace() {
            words.insert(word.to_string());
        }
        match post.body {
            Some(body) => for word in post.title.split_whitespace() {
                words.insert(word.to_string());
            },
            None => {}
        }
        words
    }

    fn get_distinct_words_in_comment(
        comment : LemmyComment
    ) -> HashSet<String> {
        HashSet::from_iter(comment.body.split_whitespace().map(|word|
            word.to_string()
        ))
    }
}
