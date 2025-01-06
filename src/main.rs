pub use self::error::{Error, Result};
use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;
use tower::ServiceBuilder;
use serde::Deserialize;
use std::net::SocketAddr;

mod error;
mod web;

#[tokio::main]
async fn main() {
    // Создаем маршруты
    let router_hello = Router::new()
        .merge(routes_hello());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("=> Listening on {addr}\n");

    // Запуск сервера
    axum::Server::bind(&addr)
        .serve(router_hello.into_make_service())
        .await
        .unwrap();
}

// Роуты для /hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handles_hello))
        .route("/hello/:name", get(handler_hello_name))
}

// fn routes_static() -> Router {
//     Router::new().nest_service(
//         "/",
//         get_service(ServeDir::new("./static"))
//             .handle_error(|err: std::io::Error| async move {
//                 (
//                     axum::http::StatusCode::INTERNAL_SERVER_ERROR,
//                     format!("Unhandled internal error: {}", err),
//                 )
//             }),
//     )
// }

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// Обработчик для /hello
async fn handles_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<22} - handles_hello - {params:?}", "Handler");

    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("<h1>Hello, {name}!</h1>"))
}

// Обработчик для /hello/:name
async fn handler_hello_name(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<22} - handler_hello_name - {name:?}", "Handler");

    Html(format!("<h1>Hello, {name}!</h1>"))
}

