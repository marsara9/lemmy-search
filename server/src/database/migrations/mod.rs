pub mod to_0_4_0;

use std::collections::HashSet;
use crate::{
    error::Result, 
    api::lemmy::models::id::LemmyId
};
use super::{
    Context, 
    dbo::get_database_client, 
    schema::{
        DatabaseSchema,
        author::Author,
        community::Community,
        site::Site, 
        posts::Post
    }
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

    pub async fn migrate(
        &self
    ) -> Result<()> {
        to_0_4_0::migrate(self.context.pool.clone())
            .await?;

        Ok(())
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
        self.update_table_column::<Post>()
            .await?;
        self.update_table_column::<LemmyId>()
            .await?;

        self.drop_table_column::<Site>()
            .await?;
        self.drop_table_column::<Author>()
            .await?;
        self.drop_table_column::<Community>()
            .await?;
        self.drop_table_column::<Post>()
            .await?;
        self.drop_table_column::<LemmyId>()
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
                    WHERE table_name = $1
                ", &[&T::get_table_name()]
            ).map(|rows| {
                rows.into_iter().map(|row| {
                    row.get(0)
                }).collect::<HashSet<String>>()
            })?;

            let mut transaction = client.transaction()?;

            let new_columns = T::get_column_types()
                .into_iter()
                .filter(|column| {
                    !existing_columns.contains(&column.0)
                }).collect::<Vec<_>>();

            println!("\tadding {} columns to '{}'", new_columns.len(), T::get_table_name());

            for (name, type_) in new_columns {

                let column_type = type_.to_sql_type_name()
                    .replace("NOT NULL", "");
                let is_nullable = type_.is_nullable();

                let add_column = format!("
                    ALTER TABLE {} ADD COLUMN {} {};
                ", T::get_table_name(), name, column_type);

                transaction.execute(&add_column, &[])?;

                if !is_nullable {

                    let set_default = format!("
                        UPDATE {} SET {} = $1
                    ", T::get_table_name(), name);

                    let default_value = *type_.get_default_value();

                    transaction.execute(&set_default, &[default_value])?;

                    let set_not_null = format!("
                        ALTER TABLE {0} ALTER COLUMN {1} SET NOT NULL;
                    ", T::get_table_name(), name);

                    transaction.execute(&set_not_null, &[])?;
                }
            }

            transaction.commit()?;
        
            Ok(())

        }).await?;

        Ok(())
    }

    pub async fn drop_table_column<T : DatabaseSchema>(
        &self
    ) -> Result<()> {
        get_database_client(&self.context.pool, |client| {

            let existing_columns = client.query(
                "SELECT column_name
                    FROM information_schema.columns 
                    WHERE table_name = $1
                ", &[&T::get_table_name()]
            ).map(|rows| {
                rows.into_iter().map(|row| {
                    row.get(0)
                }).collect::<HashSet<String>>()
            })?;

            let mut transaction = client.transaction()?;

            let old_columns = existing_columns.into_iter()
                .filter(|column| {
                    !column.starts_with("com_")
                }).filter(|column| {
                    !T::get_column_names().contains(column)
                }).collect::<Vec<_>>();

            println!("\tdropping {} columns from '{}'", old_columns.len(), T::get_table_name());

            for column in old_columns {
                let drop_column = format!("
                    ALTER TABLE {} DROP COLUMN {}
                ", T::get_table_name(), column);

                transaction.execute(&drop_column, &[])?;
            }

            transaction.commit()?;

            Ok(())

        }).await?;

        Ok(())
    }
}
