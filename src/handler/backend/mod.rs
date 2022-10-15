use axum::{routing::get, Router};

pub mod index;

pub fn router() -> Router {
    Router::new().route("/", get(index::index))
}
