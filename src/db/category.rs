use crate::model::Category;
use crate::Result;
use crate::{error::AppError, form, model::CategoryId};
use tokio_postgres::Client;

/// 创建新分类
pub async fn create(client: &Client, form: &form::CreateCategory) -> Result<CategoryId> {
    let count = super::count(
        client,
        "select count(*) from categories where name = $1",
        &[&form.name],
    )
    .await?;
    if count > 0 {
        return Err(AppError::duplicate("同名的分类已存在"));
    }
    super::insert(
        client,
        "insert into categories (name,is_del) values ($1, false) returning id",
        &[&form.name],
        "创建分类失败",
    )
    .await
}

/// 获取所有分类
pub async fn list(client: &Client) -> Result<Vec<Category>> {
    super::query(
        client,
        "select id, name, is_del from categories where is_del = false order by id asc limit 1000",
        &[],
    )
    .await
}

/// 删除或恢复分类
pub async fn del_or_restore(client: &Client, id: i32, is_del: bool) -> Result<bool> {
    let count = super::del_or_restore(client, "categories", &id, is_del).await?;
    Ok(count > 0)
}

/// 修改分类
pub async fn edit(client: &Client, form: &form::EditCategory) -> Result<bool> {
    let count = super::count(
        client,
        "select count(*) from categories where name = $1 and id <> $2",
        &[&form.name, &form.id],
    )
    .await?;
    if count > 0 {
        return Err(AppError::duplicate("同名的分类已存在"));
    }
    let count = super::execute(
        client,
        "update categories set name = $1 where id = $2",
        &[&form.name, &form.id],
    )
    .await?;
    Ok(count > 0)
}

/// 根据id查找分类
pub async fn find(client: &Client, id: i32) -> Result<Category> {
    super::query_row(
        client,
        "select id, name, is_del from categories where id = $1",
        &[&id],
    )
    .await
}
