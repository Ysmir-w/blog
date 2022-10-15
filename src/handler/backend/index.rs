use crate::handler::{log_error, render, HtmlView};
use crate::view::backend::index::Index;
use crate::Result;

pub async fn index() -> Result<HtmlView> {
    let handler_name = "backend/index/index";
    let template = Index {};
    render(template).map_err(log_error(handler_name))
}
