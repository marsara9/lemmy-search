use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NSFW_MATCH : Regex = Regex::new(r"(^| )?nsfw:(?P<enabled>(only)|(none))").unwrap();
}

pub trait NSFWFilter {
    fn get_nsfw_filter(
        &mut self
    ) -> Option<bool>;
}

impl NSFWFilter for String {
    fn get_nsfw_filter(
        &mut self
    ) -> Option<bool> {
        match NSFW_MATCH.captures(&self) {
            Some(caps) => {
                let enabled = &caps["enabled"].to_lowercase();
                let format = format!("nsfw:{}", enabled);

                *self = self.replace(&format, "");

                if enabled == "only" {
                    Some(true)
                } else if enabled == "none" {
                    Some(false)
                } else {
                    None
                }
            },
            None => None
        }.map(|value| {
            println!("\tnsfw:{}", value);
            value
        })
    }
}
