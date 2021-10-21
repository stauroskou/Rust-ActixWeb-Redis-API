use crate::DAO::configuration;
use serde::*;
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct UpdateResponse {
    updated_username: String,
    updated_password: String,
    updated_info: serde_json::Value,
}
pub async fn put_data(
    doc_to_update: &(String, String),
    updated_doc: &(String, String, serde_json::Value),
) -> UpdateResponse {
    let key_to_update: String = format!(
        "/user/?username={}&password={}",
        doc_to_update.0, doc_to_update.1
    );
    let updated_key: String = format!(
        "/user/?username={}&password={}",
        updated_doc.0, updated_doc.1
    );
    let res: serde_json::Value = json!({
        "username": updated_doc.0,
        "password": updated_doc.1,
        "info": updated_doc.2,
    });
    configuration::update_user(&key_to_update, &updated_key, &res);
    let response = UpdateResponse {
        updated_username: updated_doc.0.to_owned(),
        updated_password: updated_doc.1.to_owned(),
        updated_info: updated_doc.2.to_owned(),
    };
    response
}
