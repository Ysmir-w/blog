use crate::{handler::{HtmlView, render, log_error}, Result, view::frontend::index::Index};

pub async fn index() -> Result<HtmlView> {
    let handler_name = "frontend/index/index";
    let template = Index{};
    render(template).map_err(log_error(handler_name))
}
