use sqlx::Database;

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub data_type_detail: Option<String>,
    pub is_nullable: bool,
    pub default: Option<String>,
    pub comment: Option<String>,
    pub length: Option<u64>,
    pub max_length: Option<u64>,
    pub precision: Option<u64>,
    pub scale: Option<u64>,
    pub is_enum: bool,
    pub enum_variants: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct TableMetadata {
    pub name: String,
    pub columns: Vec<Column>,
}

pub trait FetchMetadata {
    type DB: Database;

    async fn fetch_tables(
        &self,
        pool: &sqlx::Pool<Self::DB>,
        schema: &str,
        tables: Vec<&str>,
    ) -> anyhow::Result<TableMetadata>;
}
