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
    pub seed_instance : String,
    pub log : bool
}

#[derive(Deserialize, Default, Clone)]
pub struct Postgres {
    pub user : String,
    pub password : String,
    pub hostname : String,
    pub port : u16,
    pub database : String,
    pub log : bool
}

impl Config {

    pub fn load() -> Self {
        let result = Config::from_config_file("/lemmy/config/config.yml");
        match result {
            Ok(value) => value,
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
