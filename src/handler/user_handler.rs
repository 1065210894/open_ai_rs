use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize)]
pub struct User {
    id: i32,
    name: Option<String>,
}

/// 获取用户相关接口的路由
pub fn get_router() -> Router {
    Router::new().route("/get-user-info", get(get_user_info))
}

/// 获取用户信息
pub async fn get_user_info(Query(user): Query<User>) -> Json<Value> {
    Json(json!(User {
        id: user.id,
        name: Some(String::from("张三")),
    }))
}
