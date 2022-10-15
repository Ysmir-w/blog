use std::sync::Arc;

use axum::extract::{Extension, Form, Path, Query};

use crate::db::category;
use crate::handler::{log_error, render, HtmlView};
use crate::view::backend::category::{Add, Edit, Index};
use crate::{form, AppState, Result};

use super::{get_client, redirect, Args, RedirectView};

/// 添加分类ui
pub async fn add_ui() -> Result<HtmlView> {
    let handler_name = "backend/category/add_ui";
    let template = Add {};
    render(template).map_err(log_error(handler_name))
}

/// 添加分类
pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Form(form): Form<form::CreateCategory>,
) -> Result<RedirectView> {
    let handler_name = "backend/category/add";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    category::create(&client, &form)
        .await
        .map_err(log_error(handler_name))?;
    redirect("/admin/category?msg=分类添加成功")
}

/// 分类列表
pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(args): Query<Args>,
) -> Result<HtmlView> {
    let handler_name = "backend/category/index";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let list = category::list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let template = Index {
        list,
        msg: args.message,
    };
    render(template).map_err(log_error(handler_name))
}

/// 删除分类
pub async fn del(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<RedirectView> {
    let handler_name = "backend/category/del";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    category::del_or_restore(&client, id, true)
        .await
        .map_err(log_error(handler_name))?;
    redirect("/admin/category?msg=删除分类成功")
}

/// 修改分类ui
pub async fn edit_ui(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<HtmlView> {
    let handler_name = "backend/category/edit_ui";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let item = category::find(&client, id)
        .await
        .map_err(log_error(handler_name))?;
    let template = Edit { item };
    render(template).map_err(log_error(handler_name))
}

/// 修改分类
pub async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Form(form): Form<form::EditCategory>,
) -> Result<RedirectView> {
    let handler_name = "bankend/category/edit";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    category::edit(&client, &form)
        .await
        .map_err(log_error(handler_name))?;
    redirect("/admin/category?msg=分类修改成功")
}
