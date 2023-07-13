use crate::{
    database::{DatabasePool, dbo::get_database_client}, 
    error::Result
};

pub async fn migrate(
    pool : DatabasePool
) -> Result<()> {

    get_database_client(&pool, |client| {

        client.batch_execute("
            UPDATE sites SET software = 'lemmy'
                WHERE software = '';

            ALTER TABLE posts 
                ADD COLUMN IF NOT EXISTS com_search TSVECTOR
                GENERATED ALWAYS AS	(
                    to_tsvector('english', \"name\") || ' ' || to_tsvector('english', coalesce(body, ''))
                ) stored;
            
            CREATE INDEX IF NOT EXISTS idx_search ON posts USING GIN(com_search);

            DROP TABLE IF EXISTS words;
            DROP TABLE IF EXISTS xref;
        ")?;

        Ok(())
    }).await?;

    Ok(())    
}
