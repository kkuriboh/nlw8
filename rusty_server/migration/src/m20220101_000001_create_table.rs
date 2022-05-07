use entity::*;
use sea_schema::migration::{
	prelude::*,
	sea_orm::{DbBackend, Schema},
};

#[cfg(debug_assertions)]
const BACKEND: DbBackend = DbBackend::Sqlite;
#[cfg(not(debug_assertions))]
const BACKEND: DbBackend = DbBackend::Postgres;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m20220101_000001_create_table"
	}
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(Schema::new(BACKEND).create_table_from_entity(feedback::Entity))
			.await
			.unwrap();
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(feedback::Entity).to_owned())
			.await
			.unwrap();
		Ok(())
	}
}
