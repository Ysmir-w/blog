use std::sync::Arc;

use axum::{Extension, Router};
use blog::{
    config,
    handler::{backend, frontend},
    AppState,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_rs_blog=debug");
    }
    tracing_subscriber::fmt::init();
    tracing::info!("服务已启动");

    dotenv().ok();
    let config = config::Config::from_env().expect("初始化配置失败");
    let pool = config
        .pg
        .create_pool(None, tokio_postgres::NoTls)
        .expect("创建数据库连接池失败");

    let frontend_routers = frontend::router();
    let backend_routers = backend::router();

    let app = Router::new()
        .nest("/", frontend_routers)
        .nest("/admin", backend_routers)
        .layer(Extension(Arc::new(AppState { pool })));

    axum::Server::bind(&config.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
