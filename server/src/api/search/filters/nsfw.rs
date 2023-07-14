use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NSFW_MATCH : Regex = Regex::new(r"(^| )!safeoff").unwrap();
}

pub trait NSFWFilter {
    fn get_nsfw_filter(
        &mut self
    ) -> bool;
}

impl NSFWFilter for String {
    fn get_nsfw_filter(
        &mut self
    ) -> bool {
        let result = NSFW_MATCH.is_match(&self);

        if result {
            *self = self.replace("!safeoff", "");

            println!("\tnsfw:on");
        }

        result
    }
}
