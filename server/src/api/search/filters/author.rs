use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref AUTHOR_MATCH : Regex = Regex::new(r"(^| )author:@(?P<name>\w+)@(?P<instance>[\w\-\.]+)").unwrap();
}

pub trait AuthorFilter {
    fn get_author_filter(
        &mut self
    ) -> Option<String>;
}

impl AuthorFilter for String {
    fn get_author_filter(
        &mut self
    ) -> Option<String> {
        match AUTHOR_MATCH.captures(&self) {
            Some(caps) => {
                let name = &caps["name"].to_lowercase();
                let instance = &caps["instance"].to_lowercase();
                let format = format!("author:@{}@{}", &name, &instance);

                *self = self.replace(&format, "");

                Some(format!("https://{}/u/{}", instance, name))
            },
            None => None
        }.map(|value| {
            println!("\tauthor: '{}'", value);
            value
        })
    }
}
