pub mod db;
pub mod error;
pub mod view;
pub mod handler;
pub mod model;
pub mod config;
pub mod form;

pub type Result<T> = std::result::Result<T, error::AppError>;

/// 共享状态
pub struct AppState{
    pub pool:deadpool_postgres::Pool
}