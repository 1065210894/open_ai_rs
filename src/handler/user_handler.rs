use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};
use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::global_variable::redis_client::get_redis_connection;

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
    // let mut connection = get_redis_connection();
    let user_json = json!(User {
        id: user.id,
        name: Some(String::from("张三")),
    });
    // connection.set::<&str, &str, >(format!("user_{}", user.id).as_str(), user_json.to_string().as_str())
    //     .expect("redis 写入用户消息失败！");

    Json(user_json)
}
