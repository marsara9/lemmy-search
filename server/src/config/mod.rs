use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
pub struct Config {
    pub crawler : Crawler,
    pub postgres : Postgres,
}


#[derive(Deserialize, Default, Clone)]
pub struct Crawler {
    pub enabled : bool,
    pub seed_instance : String
}

#[derive(Deserialize, Default, Clone)]
pub struct Postgres {
    pub user : String,
    pub password : String,
    pub hostname : String,
    pub database : String
}

impl Config {

    pub fn load() -> Self {
        Config::from_config_file("/lemmy/config/config.yml").unwrap_or_default()
    }

}
