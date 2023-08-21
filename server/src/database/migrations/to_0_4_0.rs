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
            UPDATE sites SET software = 'lemmy'
                WHERE software = '';

            ALTER TABLE posts 
                ADD COLUMN IF NOT EXISTS com_search TSVECTOR
                GENERATED ALWAYS AS	(
                    setweight(to_tsvector('english', \"name\"), 'B') || 
                    setweight(to_tsvector('english', coalesce(body, '')), 'A')
                ) stored;
            
            CREATE INDEX IF NOT EXISTS idx_search ON posts USING GIN(com_search);

            DROP TABLE IF EXISTS words;
            DROP TABLE IF EXISTS xref;
        ")?;

        Ok(())
    }).await?;

    Ok(())    
}
