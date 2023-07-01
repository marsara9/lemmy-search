use deadpool_r2d2::{
    InteractError, 
    PoolError, 
    Manager
};
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use tokio::task::JoinError;


#[derive(Debug)]
pub enum LemmySearchError {
    Generic(&'static str),
    Unknown(String),
    IO(std::io::Error),
    Database(postgres::Error),
    DatabaseConnection(r2d2_postgres::r2d2::Error),
    Network(reqwest::Error),
    JoinError(JoinError),
    DatabaseInteractionError(InteractError),
    DatabasePoolError(PoolError<<Manager<PostgresConnectionManager<NoTls>> as deadpool::managed::Manager>::Error>),
}

pub type Result<T> = std::result::Result<T, LemmySearchError>;

impl std::fmt::Display for LemmySearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Generic(string) => write!(f, "Error '{}'", string),
            Self::Unknown(string) => write!(f, "Unknown Error '{}'", string),
            Self::IO(err) => err.fmt(f),
            Self::Database(postgres) => postgres.fmt(f),
            Self::DatabaseConnection(r2d2_postgres) => r2d2_postgres.fmt(f),
            Self::Network(reqwest) => reqwest.fmt(f),
            Self::JoinError(join_error) => join_error.fmt(f),
            Self::DatabaseInteractionError(err) => err.fmt(f),
            Self::DatabasePoolError(err) => err.fmt(f)
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

impl From<std::io::Error> for LemmySearchError {
    fn from(value:std::io::Error) -> Self {
        LemmySearchError::IO(value)
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

impl From<JoinError> for LemmySearchError {
    fn from(value: JoinError) -> Self {
        LemmySearchError::JoinError(value)
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

