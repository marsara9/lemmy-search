use deadpool_r2d2::{
    InteractError, 
    PoolError, 
    Manager
};
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;


#[derive(Debug)]
pub enum LemmySearchError {
    Unknown(String),
    Database(postgres::Error),
    DatabaseConnection(r2d2_postgres::r2d2::Error),
    DatabaseInteractionError(InteractError),
    DatabasePoolError(PoolError<<Manager<PostgresConnectionManager<NoTls>> as deadpool::managed::Manager>::Error>),
    Network(reqwest::Error)
}

pub type Result<T> = std::result::Result<T, LemmySearchError>;

impl std::fmt::Display for LemmySearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Unknown(string) => write!(f, "Unknown Error '{}'", string),
            Self::Database(postgres) => postgres.fmt(f),
            Self::DatabaseConnection(r2d2_postgres) => r2d2_postgres.fmt(f),
            Self::Network(reqwest) => reqwest.fmt(f)
        }
    }
}

impl From<postgres::Error> for LemmySearchError {
    fn from(value: postgres::Error) -> Self {
        LemmySearchError::Database(value)
    }
}

impl From<InteractError> for LemmySearchError {
    fn from(value:InteractError) -> Self {
        LemmySearchError::DatabaseInteractionError(value)
    }
}

impl From<PoolError<<Manager<PostgresConnectionManager<NoTls>> as deadpool::managed::Manager>::Error>> for LemmySearchError {
    fn from(value:PoolError<<Manager<PostgresConnectionManager<NoTls>> as deadpool::managed::Manager>::Error>) -> Self {
        LemmySearchError::DatabasePoolError(value)
    }
}


impl From<r2d2_postgres::r2d2::Error> for LemmySearchError {
    fn from(value: r2d2_postgres::r2d2::Error) -> Self {
        LemmySearchError::DatabaseConnection(value)
    }
}

impl From<reqwest::Error> for LemmySearchError {
    fn from(value: reqwest::Error) -> Self {
        LemmySearchError::Network(value)
    }
}

pub trait LogError<T> {
    fn log_error(
        self, 
        message : 
        &str, log : bool
    ) -> Result<T>;
}

impl<T> LogError<T> for Result<T> {
    fn log_error(
        self, 
        message : &str, 
        log : bool
    ) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                println!("{}", message);
                if log {
                    println!("{}", err);
                }
                Err(err)
            }
        }
    }
}

