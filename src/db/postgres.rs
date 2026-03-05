use crate::db::{FetchMetadata, TableMetadata};

pub struct PostgresMetadataFetcher;

#[async_trait::async_trait]
impl FetchMetadata for PostgresMetadataFetcher {
    type DB = sqlx::Postgres;

    async fn fetch_tables(
        &self,
        pool: &sqlx::Pool<Self::DB>,
        schema: &str,
        tables: Vec<&str>,
    ) -> anyhow::Result<Vec<TableMetadata>> {
        anyhow::Ok(Vec::new())
    }

    async fn fetch_all_tables(
        &self,
        pool: &sqlx::Pool<Self::DB>,
        schema_filter: Option<&str>,
    ) -> anyhow::Result<Vec<TableMetadata>> {
        anyhow::Ok(Vec::new())
    }

    async fn is_data_type_enum() -> anyhow::Result<bool> {
        anyhow::Ok(false)
    }

    async fn fetch_enum_variants(
        &self,
        pool: &sqlx::Pool<Self::DB>,
        type_name: &str,
        schema: &str,
    ) -> anyhow::Result<Vec<String>> {
        anyhow::Ok(vec![])
    }
}
