use axum::extract::Json;
use serde::Deserialize;
use crate::errors::handler_errors::*;
use crate::helpers::{
    words::words_builder::WordsBuilder,
    words::words_helper::WordResult,
    jwt::*,
    config,
};

pub fn build_words(parameters: WordsPayload) -> Result<WordsBuilder, WordError> {
    match &parameters.user_token {
        Some(user_token) if !user_token.is_empty() =>
            build_words_for_reg(&parameters),
        _ => build_words_for_anon(&parameters),
    }
}

pub async fn find_words(word: String, payload: Json<SearchedWordData>) -> Vec<WordResult> {
    let limit = config::get("SEARCH_WORDS_LIMIT").parse::<i64>().unwrap();
    let default_order = config::get("DEFAULT_ORDER");
    let user_words = find_user_words(&word, payload).await;
    let exclude_id_list = user_words.iter().map(|w| w.id).collect();

    let words = WordsBuilder::new()
        .find(word.to_owned())
        .exclude_id_list(exclude_id_list)
        .order(&default_order)
        .limit(limit)
        .get_words().await;

    [user_words, words].concat()
}

async fn find_user_words(word: &String, payload: Json<SearchedWordData>) -> Vec<WordResult> {
    let mut user_words: Vec<WordResult> = Vec::new();

    if let Some(user_token) = &payload.user_token {
        if let Some(user) = get_information_from_token(&user_token) {
            user_words = find_registered_user_words(user.id, &word).await;
        }
    } else if let Some(enword_id_list) = &payload.enword_id_list {
        user_words = find_anon_user_words(enword_id_list, &word).await;
    }

    user_words
}

async fn find_registered_user_words(user_id: i32, word: &String) -> Vec<WordResult> {
    let limit = config::get("SEARCH_WORDS_LIMIT").parse::<i64>().unwrap();
    let default_order = config::get("DEFAULT_ORDER");

    let user_words = WordsBuilder::new()
        .user_id(user_id)
        .find(word.to_owned())
        .order(&default_order)
        .limit(limit)
        .get_words().await;

    user_words
        .into_iter()
        .map(|mut w| { w.is_user_word = true; w })
        .collect()
}

async fn find_anon_user_words(enword_id_list: &Vec<i32>, word: &String) -> Vec<WordResult> {
        let limit = config::get("SEARCH_WORDS_LIMIT").parse::<i64>().unwrap();
        let default_order = config::get("DEFAULT_ORDER");

        let user_words = WordsBuilder::new()
            .enword_id_list(enword_id_list.clone())
            .find(word.to_owned())
            .order(&default_order)
            .limit(limit)
            .get_words().await;

        user_words
            .into_iter()
            .map(|mut w| { w.is_user_word = true; w })
            .collect()
}

fn build_words_for_reg(parameters: &WordsPayload) -> Result<WordsBuilder, WordError> {
    let user_info = get_information_from_token(parameters.user_token.as_ref().unwrap());

    if user_info.is_none() {
        return Err(WordError::Insafficient(RequestError::InvalidUserToken));
    }

    let mut words = WordsBuilder::new().user_id(user_info.unwrap().id);

    if let Some(limit) = parameters.limit {
        words = words.limit(limit);
    }

    if let Some(page_number) = parameters.page {
        words = words.page_number(page_number);
    }

    if let Some(offset) = parameters.offset {
        words = words.offset(offset);
    }

    if let Some(exclude_id_list) = &parameters.exclude_id_list {
        words = words.exclude_id_list(exclude_id_list.clone());
    }

    words = set_order_to_words(words, parameters);
    Ok(words)
}

fn build_words_for_anon(parameters: &WordsPayload) -> Result<WordsBuilder, WordError> {
    if let Some(enword_id_list) = &parameters.enword_id_list {
        let limit: i64 = config::get("ANON_USER_WORDS_LIMIT").parse::<i64>()?;

        let mut words = WordsBuilder::new()
            .enword_id_list(enword_id_list.clone())
            .offset(0)
            .limit(limit);

        words = set_order_to_words(words, parameters);
        return Ok(words);
    }

    Err(WordError::Insafficient(RequestError::EmptyWordIdList))
}

fn set_order_to_words(words: WordsBuilder, parameters: &WordsPayload) -> WordsBuilder {
    match (&parameters.column, &parameters.direction) {
        (Some(column), _) if column == "rand" => {
            words.order(&column)
        },
        (Some(column), Some(direction)) if !column.is_empty()
            && !direction.is_empty() => {

            let order = column.to_owned() + "," + direction.as_str();
            words.order(&order)
        },
        (_, _) => words
    }
}

#[derive(Deserialize, Debug)]
pub struct WordsPayload {
    user_token: Option<String>,
    limit: Option<i64>,
    page: Option<i64>,
    offset: Option<i64>,
    column: Option<String>,
    direction: Option<String>,
    enword_id_list: Option<Vec<i32>>,
    exclude_id_list: Option<Vec<i32>>,
}

#[derive(Deserialize, Debug)]
pub struct ModifyWord {
    pub data: WordData
}

#[derive(Deserialize, Debug)]
pub struct WordData {
    pub user_token: String,
    pub word_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct SearchedWordData {
    pub user_token: Option<String>,
    pub enword_id_list: Option<Vec<i32>>,
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn get_words_anon_user() {
        let parameters = super::WordsPayload {
            user_token: None,
            limit: None,
            page: None,
            offset: None,
            column: None,
            direction: None,
            enword_id_list: Some(vec![1]),
            exclude_id_list: None,
        };

        let words = super::build_words_for_anon(&parameters)
            .unwrap().get_words().await;
        assert_eq!(words[0].value, "distinguish".to_string());
    }

    #[tokio::test]
    async fn get_words_anon_user2() {
        let enword_id_list = vec![
            438, 439, 568, 1,   578, 589, 590, 698,
        ];

        let parameters = super::WordsPayload {
            user_token: None,
            limit: None,
            page: None,
            offset: None,
            column: None,
            direction: None,
            enword_id_list: Some(enword_id_list),
            exclude_id_list: None,
        };

        let words = super::build_words_for_anon(&parameters)
            .unwrap().get_words().await;
        assert!(words.len() > 0);
    }
}
