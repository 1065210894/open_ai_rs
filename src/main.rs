#![feature(once_cell)]

mod global_variable;
mod handler;
mod user_auth;

use axum::{middleware::from_extractor, Router};

#[tokio::main]
async fn main() {
    global_variable::redis_client::get_redis_connection();
    global_variable::mysql_client::get_mysql_conn();

    // 项目总路由
    let app = Router::new()
        .merge(handler::user_handler::get_router())
        .layer(from_extractor::<user_auth::AuthenticationUser>());

    // 服务启动
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
