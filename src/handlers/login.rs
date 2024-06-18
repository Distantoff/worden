use axum::extract::Query;
use std::collections::HashMap;
use crate::errors::handler_errors::*;
use crate::helpers::oauth::service_types::ServiceType;
use crate::services::login_service::*;

pub async fn by_yandex(Query(parameters): Query<HashMap<String, String>>)
    -> Result<String, ResponseError> {
    get_token_or_create_new_user(ServiceType::Yandex, parameters).await
}

pub async fn by_vk(Query(parameters): Query<HashMap<String, String>>)
    -> Result<String, ResponseError> {
    get_token_or_create_new_user(ServiceType::VK, parameters).await
}
