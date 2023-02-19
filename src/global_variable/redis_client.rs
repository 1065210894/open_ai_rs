use super::get_system_config;
use redis::{Client, Connection};
use std::sync::OnceLock;

static REDIS_CLIENT: OnceLock<Client> = OnceLock::new();

fn init_redis_client() -> &'static Client {
    let config = &get_system_config().redis_config;
    REDIS_CLIENT.get_or_init(|| {
        Client::open(format!(
            "redis://:{}@{}:{}/{}",
            &config.password, &config.ip, &config.port, &config.database
        ))
        .unwrap()
    })
}

pub fn get_redis_connection() -> Connection {
    init_redis_client().get_connection().unwrap()
}

pub async fn get_async_redis_connection() -> redis::aio::Connection {
    init_redis_client().get_async_connection().await.unwrap()
}
