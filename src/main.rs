use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let router_hello = Router::new().route("/hello", get(handles_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("=> Listen on {addr}\n");

    axum::Server::bind(&addr)
        .serve(router_hello.into_make_service())
        .await
        .unwrap()
}
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
async fn handles_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<22} - handles_hello - {params:?}", "Handler");

    let name = params.name.as_deref().unwrap_or("world");

    Html(format!("hello <div>{name}</div>"))
}
