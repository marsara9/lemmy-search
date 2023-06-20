use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    pub crawler : Crawler,
    pub postgres : Postgres,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Crawler {
    pub enabled : bool,
    pub seed_instance : String,
    pub log : bool
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Postgres {
    pub user : String,
    pub password : String,
    pub hostname : String,
    pub port : u16,
    pub database : String,
    pub log : bool
}

impl Config {

    const DEFAULT_CONFIG_LOCATION : &str = "/lemmy/config/config.yml"; 

    pub fn load() -> Self {
        let result = Config::from_config_file(Self::DEFAULT_CONFIG_LOCATION);
        match result {
            Ok(value) => {
                println!("Config loaded from '{}'...", Self::DEFAULT_CONFIG_LOCATION);
                println!("{:?}", value);
                value
            },
            Err(_) => {
                println!("Failed to load config file...");
                println!("\tusing defaults...");
                Config {
                    ..Default::default()
                }
            }
        }
    }

}
