pub mod paginate;

use self::paginate::Paginate;

use super::Result;
use crate::error::AppError;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{
    types::{FromSqlOwned, ToSql},
    GenericClient, Statement,
};

const DEFAULT_PAGE_SIZE: u8 = 30;

/// 从数据库连接中获取Statement对象
async fn get_statement(client: &impl GenericClient, sql: &str) -> Result<Statement> {
    client.prepare(sql).await.map_err(AppError::from)
}
/// 查询多条记录
async fn query<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<Vec<T>>
where
    T: FromTokioPostgresRow,
{
    let statement = get_statement(client, sql).await?;
    let result = client
        .query(&statement, params)
        .await
        .map_err(AppError::from)?
        .iter()
        .map(|row| <T>::from_row_ref(row).unwrap())
        .collect();
    Ok(result)
}

/// 查询单条记录, 并指定当记录不存在时, 使用的可选错误信息
async fn query_row_opt<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
    message: Option<String>,
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query(client, sql, params)
        .await?
        .pop()
        .ok_or(AppError::notfound_opt(message))
}

/// 查询单条记录, 并指定当记录不存在时, 使用的错误信息
async fn query_row_message<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
    message: &str,
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query_row_opt(client, sql, params, Some(message.to_string())).await
}

/// 查询单条记录, 当记录不存在时, 使用默认的错误信息
async fn query_row<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query_row_opt(client, sql, params, None).await
}

/// 插入记录并指定返回数据
async fn insert<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
    message: &str,
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query_row_message(client, sql, params, message).await
}

/// 查询单列数据
async fn query_col<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<T>
where
    T: FromSqlOwned,
{
    let statement = get_statement(client, sql).await?;
    let result = client
        .query_one(&statement, params)
        .await
        .map_err(AppError::from)?
        .get(0);
    Ok(result)
}

async fn count(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<i64> {
    query_col(client, sql, params).await
}

async fn execute(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<u64> {
    let statement = get_statement(client, sql).await?;
    client
        .execute(&statement, params)
        .await
        .map_err(AppError::from)
}

/// 分页查询记录条数
async fn pagination<T>(
    client: &impl GenericClient,
    sql: &str,
    count_sql: &str,
    params: &[&(dyn ToSql + Sync)],
    page: u32,
) -> Result<Paginate<Vec<T>>>
where
    T: FromTokioPostgresRow,
{
    let data = query(client,sql,params).await?;
    let total_records = count(client, count_sql, params).await?;

    Ok(Paginate::new(page, DEFAULT_PAGE_SIZE, total_records, data))
}
