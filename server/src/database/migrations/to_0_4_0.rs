use crate::{
    database::{DatabasePool, dbo::get_database_client}, 
    error::Result
};

pub async fn migrate(
    pool : DatabasePool
) -> Result<()> {

    get_database_client(&pool, |client| {

        client.execute("
            UPDATE sites SET software = 'lemmy'
                WHERE software = ''
        ", &[])?;

        Ok(())
    }).await?;

    Ok(())    
}
