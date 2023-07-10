use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref COMMUNITY_MATCH : Regex = Regex::new(r" community:!(?P<name>\w+)@(?P<instance>[\w\-\.]+)").unwrap();
}

pub trait CommunityFilter {
    fn get_community_filter(
        &mut self
    ) -> Option<String>;
}

impl CommunityFilter for String {
    fn get_community_filter(
        &mut self
    ) -> Option<String> {
        match COMMUNITY_MATCH.captures(&self) {
            Some(caps) => {
                let name = &caps["name"].to_lowercase();
                let instance = &caps["instance"].to_lowercase();
                let format = format!("community:!{}@{}", &name, &instance);

                *self = self.replace(&format, "");

                Some(format!("https://{}/u/{}", instance, name))
            },
            None => None
        }        
    }
}
