use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTANCE_MATCH : Regex = Regex::new(r" instance:(?P<instance>(https://)?[\w\-\.]+)").unwrap();
}

pub trait InstanceFilter {
    fn get_instance_filter(
        &mut self
    ) -> Option<String>;
}

impl InstanceFilter for String {
    fn get_instance_filter(
        &mut self
    ) -> Option<String> {
        match INSTANCE_MATCH.captures(&self) {
            Some(caps) => {
                let cap = &caps["instance"].to_lowercase();
                let format = format!("instance:{}", cap);

                *self = self.replace(&format, "");

                Some(if cap.starts_with("https://") {
                    cap.to_string()
                } else {
                    format!("https://{}/", cap)
                })
            },
            None => None
        }        
    }
}
