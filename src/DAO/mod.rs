use once_cell::sync::Lazy;
extern crate redis;
use crate::config;
use crate::redis::Commands;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tracing::{error, info, instrument};

static DATABASE_CONNECTION: Lazy<Mutex<redis::Connection>> = Lazy::new(|| {
    let conf = config::Config::get_config();
    let redis: String;
    match conf {
        Ok(conf) => {
            redis = conf.redis.to_string();
        }
        Err(_err) => redis = String::new(),
    }
    let redis_client = redis::Client::open(redis).unwrap();
    let redis_connection = redis_client.get_connection().unwrap();
    Mutex::new(redis_connection)
});
#[derive(Serialize, Deserialize)]
pub struct PostResponse {
    deleted: u32,
    success: bool,
}
pub struct configuration {}

impl configuration {
    #[instrument]
    pub fn get_user(key: &String) -> serde_json::Value {
        let con = DATABASE_CONNECTION.lock();
        let mut values: Vec<serde_json::Value> = Vec::new();
        match con {
            Ok(mut connection) => {
                info!("Connection established");
                let keys: Vec<String> = connection.keys(key).unwrap_or_else(|e| {
                    sentry::capture_error(&e);
                    error!("{}", e);
                    let empty_vec: Vec<String> = Vec::new();
                    empty_vec
                });
                for i in keys.iter() {
                    let val: Result<String, redis::RedisError> = connection.get(i);
                    match val {
                        Ok(value) => {
                            let json = serde_json::from_str(&value);
                            match json {
                                Ok(json_val) => {
                                    values.push(json_val);
                                }
                                Err(err) => {
                                    sentry::capture_error(&err);
                                    error!("{}", err);
                                }
                            }
                        }
                        Err(err) => {
                            sentry::capture_error(&err);
                            error!("{}", err);
                        }
                    }
                }
                info!("get_config completed successfully");
            }
            Err(error) => {
                sentry::capture_error(&error);
                error!("{}", error);
            }
        }
        let val_json = serde_json::to_value(values).unwrap_or_else(|e| {
            sentry::capture_error(&e);
            error!("{}", e);
            let empty_json: serde_json::Value = serde_json::Value::Null;
            empty_json
        });
        val_json
    }
    #[instrument]
    pub fn create_user(key: &String, value: &serde_json::Value) {
        let con = DATABASE_CONNECTION.lock();
        match con {
            Ok(mut connection) => {
                info!("Connection established");
                let val = serde_json::to_string(&value);
                match val {
                    Ok(json) => {
                        let set: Result<String, redis::RedisError> = connection.set(key, json);
                        match set {
                            Ok(_) => {
                                info!("Created new configuration");
                            }
                            Err(error) => {
                                sentry::capture_error(&error);
                                error!("{}", error);
                            }
                        }
                    }
                    Err(error) => {
                        sentry::capture_error(&error);
                        error!("{}", error);
                    }
                }
            }
            Err(error) => {
                sentry::capture_error(&error);
                error!("{}", error);
            }
        }
    }
    #[instrument]
    pub fn delete_user(key: &String) -> PostResponse {
        let con = DATABASE_CONNECTION.lock();
        let mut deleted = 0;
        match con {
            Ok(mut connection) => {
                let del: Result<u32, redis::RedisError> = connection.del(key);
                match del {
                    Ok(val) => {
                        deleted = val;
                    }
                    Err(error) => {
                        sentry::capture_error(&error);
                        error!("{}", error);
                    }
                }
            }
            Err(error) => {
                sentry::capture_error(&error);
                error!("{}", error);
            }
        }
        info!("Delete items: {}", deleted);
        PostResponse {
            deleted: deleted,
            success: true,
        }
    }
    #[instrument]
    pub fn update_user(key: &String, updated_key: &String, updated_config: &serde_json::Value) {
        let con = DATABASE_CONNECTION.lock();
        match con {
            Ok(mut connection) => {
                let config = serde_json::to_string(&updated_config);
                match config {
                    Ok(val) => {
                        let set: Result<String, redis::RedisError> = connection.set(key, val);
                        match set {
                            Ok(_) => {}
                            Err(error) => {
                                sentry::capture_error(&error);
                                error!("{}", error);
                            }
                        }
                        let rename: Result<String, redis::RedisError> =
                            connection.rename(key, updated_key);
                        match rename {
                            Ok(_) => {}
                            Err(error) => {
                                sentry::capture_error(&error);
                                error!("{}", error);
                            }
                        }
                        info!("updated_config completed successfully");
                    }
                    Err(error) => {
                        sentry::capture_error(&error);
                        error!("{}", error);
                    }
                }
            }
            Err(error) => {
                sentry::capture_error(&error);
                error!("{}", error);
            }
        }
    }
}
