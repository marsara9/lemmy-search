use std::time::Duration;

use deadpool::Runtime;
use diesel_async::{
    AsyncPgConnection, 
    pooled_connection::{
        deadpool::{Pool, BuildError}, 
        AsyncDieselConnectionManager
    }
};

use crate::config::Postgres;

pub mod dbo;

pub struct Database {
    connection_string : String
}

pub type DatabasePool = Pool<AsyncPgConnection>;

impl Database {

    const POOL_TIMEOUT: Option<Duration> = Some(Duration::from_secs(5));
    
    pub fn new(config : Postgres) -> Self {
        let connection_string = format!(
            "postgresql://{}:{}@{}/{}",
            config.user,
            config.password,
            config.hostname,
            config.database
        );

        Database {
            connection_string
        }
    }

    pub async fn build_database_pool(
        &self
    ) -> Result<DatabasePool, BuildError> {
        
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&self.connection_string);
        let pool = Pool::builder(manager)
          .max_size(5)
          .wait_timeout(Self::POOL_TIMEOUT)
          .create_timeout(Self::POOL_TIMEOUT)
          .recycle_timeout(Self::POOL_TIMEOUT)
          .runtime(Runtime::Tokio1)
          .build()?;

        Ok(pool)
      }
}
