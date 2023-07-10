use std::collections::HashSet;
use crate::{
    error::Result, 
    api::lemmy::models::{
        post::PostData, 
        author::Author, 
        community::Community, 
        id::LemmyId
    }
};
use super::{
    Context, 
    dbo::get_database_client, 
    schema::{DatabaseSchema, site::Site, word::Word, xref::Search}
};

pub struct DatabaseMigrations {
    context : Context
}

impl DatabaseMigrations {
    pub fn new(context : Context) -> Self {
        Self {
            context
        }
    }

    pub async fn update_table_columns(
        &self
    ) -> Result<()> {
        self.update_table_column::<Site>()
            .await?;
        self.update_table_column::<Author>()
            .await?;
        self.update_table_column::<Community>()
            .await?;
        self.update_table_column::<PostData>()
            .await?;
        self.update_table_column::<LemmyId>()
            .await?;
        self.update_table_column::<Word>()
            .await?;
        self.update_table_column::<Search>()
            .await?;

        Ok(())
    }

    /**
     * Adds any new columns that aren't on the existing database.
     */
    async fn update_table_column<T : DatabaseSchema>(
        &self
    ) -> Result<()> {
        get_database_client(&self.context.pool, |client| {

            let existing_columns = client.query(
                "SELECT column_name
                    FROM information_schema.columns 
                    WHERE table_name = '%1'
                ", &[&T::get_table_name()]
            ).map(|rows| {
                rows.into_iter().map(|row| {
                    row.get(0)
                }).collect::<HashSet<String>>()
            })?;

            let new_columns = T::get_column_types()
                .into_iter()
                .filter(|column| {
                    existing_columns.contains(&column.0)
                })
                .into_iter()
                .map(|column| {

                    let add_column = format!("
                        ALTER TABLE {0} ADD COLUMN {1} {2} DEFAULT %1;
                    ", T::get_table_name(), column.0, column.1.to_sql_type_name());

                    let drop_default = format!("
                        ALTER TABLE {0} ALTER COLUMN {1} DROP DEFAULT;
                    ", T::get_table_name(), column.0);

                    (add_column, drop_default, column.1.clone())
                }).collect::<Vec<_>>();

            let mut transaction = client.transaction()?;
            for (add_column, drop_default, default_value) in new_columns {
                transaction.execute(&add_column, &[*default_value.get_default_value()])?;
                transaction.execute(&drop_default, &[])?;
            }
            transaction.commit()?;

            Ok(())

        }).await?;

        Ok(())
    }
}
