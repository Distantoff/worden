use crate::errors::handler_errors::*;
use crate::helpers::oauth::{OAuth, service_types::ServiceType};
use crate::helpers::jwt::*;
use crate::models::users::Users;
use crate::helpers::modification::users::UserModification;
use std::collections::HashMap;

pub async fn get_token_or_create_new_user(service_type: ServiceType, parameters: HashMap<String, String>) -> Result<String, ResponseError> {
    let oauth = OAuth::new(service_type);
    let token = if let Some(email) = oauth.fetch_email(parameters).await {
        let user = match Users::get_user_info_by_email(&email) {
            Some(user) => user,
            None => Users::create_user_by_email(&email).unwrap()
        };

        Ok(generate_token(user.id, &user.email))
    } else {
        Err(ResponseError::InvalidParsingJWT)
    };

    token
}
