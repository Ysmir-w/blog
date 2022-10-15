use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "categories")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub is_del: bool,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "categories")]
pub struct CategoryId {
    pub id: i32,
}
