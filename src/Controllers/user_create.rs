use crate::DAO::configuration;
use nanoid::nanoid;
use serde::*;
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct PostResponse {
    inserted: bool,
    username: String,
}

pub async fn post_data(
    username: String,
    password: String,
    info: serde_json::Value,
) -> PostResponse {
    let id = nanoid!();
    let key_with_max_ver: String = format!("/user/?username={}&password={}", username, password);
    let res: serde_json::Value = json!({
        "_id": id,
        "username": username,
        "password": password,
        "info": info,
    });
    configuration::create_user(&key_with_max_ver, &res);
    let resp = PostResponse {
        inserted: true,
        username: username,
    };
    resp
}
