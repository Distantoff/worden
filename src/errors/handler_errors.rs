use axum::{
    http::StatusCode,
    response::{ IntoResponse, Response },
};
use std::{
    error::Error,
    fmt::{ Display, Formatter, Result },
};
use diesel::result;

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "incorrect data has been sent")
    }
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::InvalidParsingJWT =>
                write!(f, "Invalid Parsing JWT. Can't authorize"),
            Self::WordNotFound =>
                write!(f, "Word not found"),
            Self::CreateNewWordError(err) =>
                write!(f, "Error at attempt to create a new word. {}", err.to_string()),
        }
    }
}

impl From<result::Error> for ResponseError {
    fn from(error: result::Error) -> Self {
        Self::CreateNewWordError(error)
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        let body = match self {
            Self::InvalidParsingJWT => "Invalid Parsing JWT. Can't authorize",
            Self::WordNotFound => "Word not found",
            Self::CreateNewWordError(_) => "Error at attempt to create a new word",
        };

        (StatusCode::UNAUTHORIZED, body).into_response()
    }
}

impl From<std::num::ParseIntError> for WordError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::Parsing(error)
    }
}

#[derive(Debug)]
pub enum RequestError {
    EmptyWordIdList,
    InvalidUserToken,
}

#[derive(Debug)]
pub enum ResponseError {
    InvalidParsingJWT,
    WordNotFound,
    CreateNewWordError(result::Error)
}

#[derive(Debug)]
pub enum WordError {
    Parsing(std::num::ParseIntError),
    Insafficient(RequestError),
}

impl Error for RequestError { }
impl Error for ResponseError { }
