use axum::{
    extract::Path,
    http::header::{self},
    response::{ IntoResponse, AppendHeaders },
};
use crate::services::file_service;


pub async fn audio(Path((lang, filename)): Path<(String, String)>) -> impl IntoResponse {
    let header_content_disposition = format!("attachment; filename=\"{}\"", filename);
    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "audio/mpeg"),
        (header::CONTENT_DISPOSITION, &header_content_disposition),
    ]);

    let mp3 = file_service::get_mp3(&lang, &filename).await;
    (headers, mp3).into_response()
}
