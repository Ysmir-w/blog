pub mod db;
pub mod error;
pub mod view;
pub mod handler;

pub type Result<T> = std::result::Result<T, error::AppError>;
