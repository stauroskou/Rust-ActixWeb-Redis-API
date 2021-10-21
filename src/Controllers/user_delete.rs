use crate::DAO::*;
pub async fn delete_data(key: &String) -> PostResponse {
    let response = configuration::delete_user(&key);
    response
}
