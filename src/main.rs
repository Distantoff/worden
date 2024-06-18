mod app;
mod errors;
mod handlers;
mod helpers;
mod models;
mod routes;
mod services;
mod schema;

use std::net::SocketAddr;
use crate::routes::routes;
use crate::helpers::config;

#[tokio::main]
async fn main() {
    let ports = app::ports();

    if config::is_debug_build() {
        tokio::spawn(async move {
            let address_http = SocketAddr::from(([0, 0, 0, 0], ports.http));
            axum_server::bind(address_http)
                .serve(routes().into_make_service()).await.unwrap()
        });
    } else {
        tokio::spawn(app::redirect_http_to_https(ports));
    }

    let address = SocketAddr::from(([0, 0, 0, 0], ports.https));
    axum_server::bind_rustls(address, app::tls_config().await)
        .serve(routes().into_make_service())
        .await
        .unwrap();
}
