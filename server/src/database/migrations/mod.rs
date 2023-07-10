use crate::error::Result;

use super::Context;


pub struct DatabaseMigrations {
    context : Context
}

impl DatabaseMigrations {
    pub fn new(context : Context) -> Self {
        Self {
            context
        }
    }

    pub async fn to0_4_0(
        &self
    ) -> Result<()> {
        Ok(())
    }
}
