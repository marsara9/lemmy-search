use config_file::FromConfigFile;
use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    pub development_mode : bool,
    pub crawler : Crawler,
    pub postgres : Postgres,
    pub donations : Donations
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Crawler {
    pub enabled : bool,
    pub seed_instance : String,
    pub single_instance_only : Option<bool>,
    pub log : bool
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Postgres {
    pub user : String,
    pub password : String,
    pub hostname : String,
    pub port : u16,
    pub database : String,
    pub log : bool,
    pub max_size : usize
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Donations {
    pub text : Option<String>,
    pub url : Option<String>
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
            Err(err) => {
                println!("Failed to load config file...");
                println!("{}", err);
                Config {
                    ..Default::default()
                }
            }
        }
    }

}
