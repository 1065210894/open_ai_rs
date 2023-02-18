use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::OnceLock;

pub mod mysql_client;
pub mod redis_client;

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Deserialize, Debug, Serialize)]
struct MysqlConfig {
    ip: String,
    port: String,
    user: String,
    password: String,
    database: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct RedisConfig {
    ip: String,
    port: String,
    password: String,
    database: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    mysql_config: MysqlConfig,
    redis_config: RedisConfig,
}

/// 获取系统配置项
fn get_system_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let env_json = json!(get_config());
        println!("当前项目环境配置Json：{}", env_json.to_string());
        serde_json::from_value::<Config>(env_json).expect("转化环境配置失败")
    })
}

/// 自动通过环境获取配置
fn get_config() -> HashMap<String, HashMap<String, String>> {
    let mut default_config: HashMap<String, HashMap<String, String>> = toml::from_str(
        &read_to_string("src/config/config.toml")
            .expect("Something went wrong with reading config.toml..."),
    )
    .expect("read config.toml error");

    let active_env = default_config
        .get("config")
        .unwrap()
        .get("active_env")
        .unwrap();

    // 读取按环境需要替换或补充的配置
    let waite_insert_config: Result<HashMap<String, HashMap<String, String>>, _> = toml::from_str(
        &read_to_string(format!("src/config/config-{}.toml", active_env)).expect(&format!(
            "Something went wrong with reading config-{}.toml...",
            active_env
        )),
    );

    if let Ok(item_config) = waite_insert_config {
        // 遍历对应环境的配置项与配置
        for (item_name, item) in item_config.iter() {
            // 遍历配置项中的配置
            for (config_name, config_value) in item.iter() {
                // 如果该配置在默认的配置中找不到就添加配置，找到则修改配置
                if let Some(config_map) = default_config.get_mut(item_name) {
                    //成功找到配置后，进行修改
                    config_map.insert(config_name.clone(), config_value.clone());
                } else {
                    // 如果找不到配置项那么就新建配置项并配置内容然后添加到默认配置中
                    let mut config_hash_map: HashMap<String, String> = HashMap::new();
                    config_hash_map.insert(config_name.clone(), config_value.clone());
                    // 将新建的配置项导入默认配置中
                    default_config.insert(item_name.clone(), config_hash_map);
                }
            }
        }
    }

    default_config
}
