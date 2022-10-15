use askama::Template;
use axum::response::Html;

use crate::error::AppError;
use crate::Result;
pub mod frontend;
pub mod backend;

type HtmlView = axum::response::Html<String>;

fn render<T>(template: T) -> Result<HtmlView>
where
    T: Template,
{
    let html = template.render().map_err(AppError::from)?;
    Ok(Html(html))
}

fn log_error(handler_name: &str) -> Box<dyn Fn(AppError) -> AppError> {
    let handler_name = String::from(handler_name);
    Box::new(move |err| {
        tracing::error!("操作失败: {:?}, {}", err, handler_name);
        err
    })
}
