use axum::extract::{Json, Path, rejection::JsonRejection};
use crate::errors::handler_errors::*;
use crate::models::users::Users;
use crate::helpers::modification::users::UserModification;
use crate::helpers::jwt::*;
use crate::services::word_service::*;

pub async fn index(
    payload: Result<Json<WordsPayload>, JsonRejection>) -> String {

    match payload {
        Ok(Json(parameters)) => {
            let json = match build_words(parameters) {
                Ok(words) => {
                    let user_words = words.get_words().await;
                    serde_json::to_string(&user_words).unwrap()
                },
                Err(_) => String::from("[]")
            };

            json
        },
        Err(err) => format!("Error {}", err.to_string())
    }
}

pub async fn find(Path(word): Path<String>,
    payload: Result<Json<SearchedWordData>, JsonRejection>) -> String {

    let payload = payload.expect("error to get data from extractor");
    let words = find_words(word, payload).await;
    serde_json::to_string(&words).unwrap()
}

pub async fn add(payload: Result<Json<ModifyWord>, JsonRejection>) -> String {
    match payload {
        Ok(Json(parameters)) => {
            if let Some(user) = get_information_from_token(&parameters.data.user_token) {
                return Users::add_word(user.id, parameters.data.word_id)
                    .unwrap().to_string();
            }
            "Error during to get user information from token".to_string()
        },
        Err(err) => format!("Error {}", err.to_string())
    }
}

pub async fn delete(payload: Result<Json<WordData>, JsonRejection>) -> String {
    match payload {
        Ok(Json(parameters)) => {
            if let Some(user) = get_information_from_token(&parameters.user_token) {
                return Users::hide_word(user.id, parameters.word_id)
                    .unwrap().to_string();
            }
            "Error during to get user information from token".to_string()
        },
        Err(err) => format!("Error {}", err.to_string())
    }
}

pub async fn fetch_and_save_word(Path(word): Path<String>)
    -> Result<String, ResponseError> {
    use crate::helpers::words::words_helper::WordsHelper;
    use crate::helpers::modification::{
        words::WordModification, yandex_puller};

    if let Some(word_json) = yandex_puller::get_word(&word).await {
        return Ok(WordsHelper::add_word(word_json).await?.to_string());
    }

    Err(ResponseError::WordNotFound)
}
