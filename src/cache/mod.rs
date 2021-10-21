use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;
use tokio::time::{sleep, Duration};

static CACHE: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let hashmap = HashMap::new();
    RwLock::new(hashmap)
});

pub struct Configuration {}
impl Configuration {
    pub fn store_in_cache(key: &String, value: &serde_json::Value) {
        let mut insert_in_cache = CACHE.write().unwrap();
        insert_in_cache.insert(key.to_string(), serde_json::to_string(&value).unwrap());
    }
    pub fn get_from_cache(key: &String) -> serde_json::Value {
        let read_from_cache = CACHE.read().unwrap();
        let data = read_from_cache.get(&key.to_string()).unwrap();
        let d: serde_json::Value = serde_json::from_str(&data).unwrap();
        d
    }
    pub fn exists_in_cache(key: &String) -> bool {
        let read_cache = CACHE.read().unwrap();
        let key_exists = read_cache.contains_key(&key.to_string());
        key_exists
    }
    pub async fn init(secs: u64) {
        loop {
            sleep(Duration::from_secs(secs)).await;
            CACHE.write().unwrap().clear();
        }
    }
}
