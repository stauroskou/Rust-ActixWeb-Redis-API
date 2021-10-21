use crate::cache::Configuration;
use crate::DAO::configuration;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    service_name: Option<String>,
    enviroment: Option<String>,
    version: Option<u32>,
}
pub async fn get_all(key: &String) -> serde_json::Value {
    if Configuration::exists_in_cache(&key) {
        let conf = Configuration::get_from_cache(&key);
        conf
    } else {
        let conf = configuration::get_user(&key);
        Configuration::store_in_cache(&key, &conf);
        conf
    }
}
