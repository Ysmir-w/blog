pub mod db;
pub mod error;

pub type Result<T> = std::result::Result<T, error::AppError>;
