use axum::{
    Router,
    routing::get,
    routing::post,
    routing::put,
    routing::delete,
};
use tower_http::services::{ServeDir, ServeFile};
use crate::helpers::config;
use crate::handlers::{login, words, users, files};

pub fn routes() -> Router {
    let index = config::get("ROOT_DIR") + "dist/main.html";
    let print_page = config::get("ROOT_DIR") + "dist/print.html";
    let assets = config::get("ROOT_DIR") + "dist/assets";

    Router::new()
        .route("/api/words", post(words::index))
        .route("/api/words", put(words::add))
        .route("/api/words", delete(words::delete))
        .route("/api/words/:name", post(words::find))
        .route("/api/words/:name", put(words::fetch_and_save_word))

        .route("/api/users/:user_token", get(users::get_by_token))
        .route("/api/yandex", get(login::by_yandex))
        .route("/api/vk", get(login::by_vk))
        .route("/engine/audio/mp3/:lang/:name", get(files::audio))

        .nest_service("/assets", ServeDir::new(assets))
        .nest_service("/", ServeFile::new(index))
        .nest_service("/print", ServeFile::new(print_page))
}
