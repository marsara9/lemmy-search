use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub crawler : Crawler,
    pub postgres : Postgres,
}


#[derive(Deserialize)]
pub struct Crawler {
    pub seed_instance : String
}

#[derive(Deserialize)]
pub struct Postgres {
    pub location : String,
}

impl Config {

    pub fn load() -> Self {
        Config::from_config_file("config/config.yml").unwrap()
    }

}
