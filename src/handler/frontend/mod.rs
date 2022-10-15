use axum::{Router, routing::get};

pub mod index;

pub fn router()->Router{
    Router::new().route("/", get(index::index))
}