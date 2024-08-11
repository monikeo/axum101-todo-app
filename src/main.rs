use axum::Router;
use axum::routing::{get,post};
use axum::response::IntoResponse;
use axum::response::Html;
use axum::extract::Query;
use std::collections::HashMap;

async fn index_handler() -> impl IntoResponse {
    Html("<p>Hello todolist</p>")
}

async fn todos_handler(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    Html("<p>Hello from here</p>")
}

async fn app() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/todos", get(todos_handler).post(todos_handler))
}

#[tokio::main]
async fn main() {
    let app = app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();
    axum::serve(listener, app).await.unwrap()

}


