use crate::{
    database::{
        DatabasePool, 
        dbo::get_database_client
    }, 
    error::Result
};

pub async fn migrate(
    pool : DatabasePool
) -> Result<()> {

    get_database_client(&pool, |client| {

        client.batch_execute("
            DROP TABLE IF EXISTS lemmy_ids;
        ")?;

        Ok(())
    }).await?;

    Ok(())    
}
