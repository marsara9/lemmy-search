use std::collections::HashSet;

use super::Crawler;
use lemmy_search_common::{
    models::LemmyPost
};

trait Analyizer {
    fn get_distinct_words(
        post : LemmyPost
    ) -> HashSet<String>;
}

impl Analyizer for Crawler {

    fn get_distinct_words(
        post : LemmyPost
    ) -> HashSet<String> {
        HashSet::from_iter(post.title.split_whitespace().map(|word|
            word.to_string()
        ))
    }
}
