use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NSFW_MATCH : Regex = Regex::new(r" nsfw:(?P<yes_no>(yes)|(no))").unwrap();
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
                let yes_no = &caps["yes_no"].to_lowercase();
                let format = format!("nsfw:{}", yes_no);

                *self = self.replace(&format, "");

                Some(yes_no == "yes")
            },
            None => None
        }.map(|value| {
            println!("\tnsfw:{}", value);
            value
        })
    }
}
