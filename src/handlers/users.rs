use crate::helpers::jwt::*;
use axum::extract::Path;

pub async fn get_by_token(Path(user_token): Path<String>) -> String {
    if let Some(user) = get_information_from_token(&user_token) {
        return serde_json::to_string(&user).unwrap();
    }
    String::new()
}
