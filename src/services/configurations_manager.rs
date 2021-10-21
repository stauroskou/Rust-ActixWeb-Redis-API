use actix_web::{web, HttpResponse, Result};
use serde::*;
extern crate redis;
#[path = "../Controllers/mod.rs"]
mod Controllers;
use crate::DAO::*;

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
pub struct PostQueryParams {
    username: String,
    password: String,
    info: serde_json::Value,
}
#[derive(Serialize, Deserialize)]
pub struct PutQueryParams {
    username: String,
    password: String,
    info: serde_json::Value,
}
#[derive(Serialize, Deserialize)]
pub struct Response {
    success: bool,
}

pub async fn get_user(doc: web::Json<QueryParams>) -> Result<HttpResponse> {
    let key: String = format!("/user/?username={}&password={}", doc.username, doc.password);
    let config = Controllers::user_get::get_all(&key).await;
    let response = Ok(HttpResponse::Ok().json(&config));
    response
}

pub async fn delete_user(doc: web::Json<QueryParams>) -> Result<HttpResponse> {
    let key: String = format!("/user/?username={}&password={}", doc.username, doc.password);
    let response: PostResponse = Controllers::user_delete::delete_data(&key).await;
    Ok(HttpResponse::Ok().json(response))
}

pub async fn post_user(info: web::Json<PostQueryParams>) -> Result<HttpResponse> {
    let resp = Controllers::user_create::post_data(
        info.username.to_owned(),
        info.password.to_owned(),
        info.info.to_owned(),
    )
    .await;
    Ok(HttpResponse::Ok().json(&resp))
}
pub async fn put_user(
    doc_to_update: web::Query<QueryParams>,
    updated_doc: web::Json<PutQueryParams>,
) -> Result<HttpResponse> {
    let doc_up: (String, String) = (
        doc_to_update.username.to_owned(),
        doc_to_update.password.to_owned(),
    );
    let up_doc: (String, String, serde_json::Value) = (
        updated_doc.username.to_owned(),
        updated_doc.password.to_owned(),
        updated_doc.info.to_owned(),
    );
    let response = Controllers::user_update::put_data(&doc_up, &up_doc).await;
    Ok(HttpResponse::Ok().json(response))
}
