use crate::{error::AppError, AppState, Result};
use axum::{
    http::{header, HeaderMap, StatusCode},
    routing::get,
    Router,
};
use deadpool_postgres::Client;
use serde::Deserialize;

pub mod category;
pub mod index;

type RedirectView = (StatusCode, HeaderMap, ());

pub fn router() -> Router {
    let category_router = Router::new()
        .route("/", get(category::index))
        .route("/add", get(category::add_ui).post(category::add))
        .route("/del/:id", get(category::del))
        .route("/edit/:id", get(category::edit_ui).post(category::edit));

    Router::new()
        .route("/", get(index::index))
        .nest("/category", category_router)
}

fn redirect(url: &str) -> Result<RedirectView> {
    let mut hm = HeaderMap::new();
    hm.append(header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, hm, ()))
}

async fn get_client(state: &AppState) -> Result<Client> {
    state.pool.get().await.map_err(AppError::from)
}

#[derive(Deserialize)]
pub struct Args {
    pub message: Option<String>,
    pub page: Option<u32>,
}

impl Args {
    pub fn message(&self) -> String {
        self.message.clone().unwrap_or(String::new())
    }

    pub fn page(&self) -> u32 {
        self.page.unwrap_or(0)
    }
}
