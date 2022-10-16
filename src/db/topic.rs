use std::time;

use tokio_postgres::Client;

use crate::{
    form::{self, EditTopic},
    md,
    model::{TopicEditData, TopicID, TopicList},
    Result,
};

use super::{paginate::Paginate, DEFAULT_PAGE_SIZE};

pub async fn create(client: &Client, form: &form::CreateTopic) -> Result<TopicID> {
    let html = md2html(&form.markdown);
    let dateline = time::SystemTime::now();
    super::insert(client, "INSERT INTO topics (title,category_id, summary, markdown, html, hit, dateline, is_del) VALUES ($1, $2, $3, $4, $5, 0, $6, false) RETURNING id", &[&form
    .title, &form.category_id, &form.summary, &form.markdown, &html,  &dateline ], "添加文章失败").await
}

pub async fn list(client: &Client, page: u32) -> Result<Paginate<Vec<TopicList>>> {
    let sql=format!("SELECT id,title,category_id,summary,hit,dateline,is_del,category_name FROM v_topic_cat_list WHERE is_del=false ORDER BY id DESC LIMIT {} OFFSET {}", DEFAULT_PAGE_SIZE, DEFAULT_PAGE_SIZE as u32 * page);
    let count_sql = "SELECT COUNT(*) FROM v_topic_cat_list WHERE is_del=false";
    super::pagination(client, &sql, count_sql, &[], page).await
}

pub async fn update(client: &Client, form: &EditTopic, id: i64) -> Result<bool> {
    let html = md2html(&form.markdown);
    let sql =
        "UPDATE topics SET title=$1,category_id=$2,summary=$3,markdown=$4,html=$5 WHERE id=$6";
    let count = super::execute(
        client,
        sql,
        &[
            &form.title,
            &form.category_id,
            &form.summary,
            &form.markdown,
            &html,
            &id,
        ],
    )
    .await?;
    Ok(count > 0)
}

pub async fn find2edit(client: &Client, id: i64) -> Result<TopicEditData> {
    super::query_row(
        client,
        "select id,title,category_id,summary,markdown from topics where id = $1 limit 1",
        &[&id],
    )
    .await
}

pub async fn del_or_restore(client: &Client, id: i64, is_del: bool) -> Result<bool> {
    let count = super::del_or_restore(client, "topics", &id, is_del).await?;
    Ok(count > 0)
}

fn md2html(markdown: &str) -> String {
    md::to_html(markdown)
}
