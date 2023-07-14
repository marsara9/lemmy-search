use chrono::{
    Utc, 
    DateTime, NaiveDate
};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SINCE_MATCH : Regex = Regex::new(r"(^| )since:(?P<date>\d{4}\-(0?[1-9]|1[012])\-(0?[1-9]|[12][0-9]|3[01]))").unwrap();
    static ref UNTIL_MATCH : Regex = Regex::new(r"(^| )until:(?P<date>\d{4}\-(0?[1-9]|1[012])\-(0?[1-9]|[12][0-9]|3[01]))").unwrap();
}

pub trait DateFilter {
    fn get_since_filter(
        &mut self
    ) -> Option<DateTime<Utc>>;

    fn get_until_filter(
        &mut self
    ) -> Option<DateTime<Utc>>;
}

impl DateFilter for String {
    fn get_since_filter(
        &mut self
    ) -> Option<DateTime<Utc>> {
        match SINCE_MATCH.captures(&self) {
            Some(caps) => {
                let date = &caps["date"].to_lowercase();
                let format = format!("since:{}", &date);

                *self = self.replace(&format, "");

                NaiveDate::parse_from_str(date, "%F")
                    .ok()
                    .map(|n| {
                        n.and_hms_opt(0, 0, 0)
                    })
                    .flatten()
                    .map(|n| {
                        n.and_utc()
                    })
            },
            None => None
        }.map(|value| {
            println!("\tsince: '{}'", value);
            value
        })
    }

    fn get_until_filter(
        &mut self
    ) -> Option<DateTime<Utc>> {
        match UNTIL_MATCH.captures(&self) {
            Some(caps) => {
                let date = &caps["date"].to_lowercase();
                let format = format!("until:{}", &date);

                *self = self.replace(&format, "");

                NaiveDate::parse_from_str(date, "%F")
                    .ok()
                    .map(|n| {
                        n.and_hms_opt(0, 0, 0)
                    })
                    .flatten()
                    .map(|n| {
                        n.and_utc()
                    })
            },
            None => None
        }.map(|value| {
            println!("\tuntil: '{}'", value);
            value
        })
    }
}
