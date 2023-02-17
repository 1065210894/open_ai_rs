mod open_ai;
mod model_handler;
mod middleware;
mod user_auth;

use axum::{
    Router,
    middleware::from_extractor,
};

#[tokio::main]
async fn main() {
    // 项目总路由
    let app = Router::new()
        .merge(model_handler::user_handler::get_router())
        .layer(from_extractor::<user_auth::AuthenticationUser>());

    // 服务启动
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
