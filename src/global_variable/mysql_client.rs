use super::get_system_config;
use mysql::{Pool, PooledConn};
use std::sync::OnceLock;

static MYSQL_POOL: OnceLock<Pool> = OnceLock::new();

/// 初始化话mysql连接池，但是需要进行全局变量初始化
fn init_mysql_pool() -> &'static Pool {
    MYSQL_POOL.get_or_init(|| {
        let config = &get_system_config().mysql_config;
        let password = der(&config.password);
        Pool::new(
            format!(
                "mysql://{}:{}@{}:{}/{}",
                &config.user, &password, &config.ip, &config.port, &config.database
            )
            .as_str(),
        )
        .unwrap()
    })
}

fn der(password: &str) -> String {
    String::from(password)
}

/// 获取mysql的一个连接
pub fn get_mysql_conn() -> PooledConn {
    init_mysql_pool().get_conn().unwrap()
}
