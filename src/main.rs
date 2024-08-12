use axum::Router;
use axum::routing::{get,post};
use axum::response::IntoResponse;
use axum::response::Html;
use axum::extract::Query;
use std::collections::HashMap;
use axum::Json;
use serde::{Deserialize, Serialize};
use axum::http::status::StatusCode;
use jsonwebtoken as jwt;


const SECRET: &[u8] = b"god_token";

#[derive(Deserialize, Serialize)]
struct Todo {
    id: u8,
    name: String,
}

impl Todo {
    fn new(id: u8, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct StatusMessage{
    message: String,
    code: String,
}

impl StatusMessage {
    fn new(message: &str, code: &str) -> Self {
        Self {
            message: message.to_string(),
            code: code.to_string()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    id: usize,
    name: String,
}

impl Claims {
    fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: name.to_string()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LoginResponse {
    token: String
}

impl LoginResponse {
    fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}

async fn index_handler() -> impl IntoResponse {
    Html("<p>Hello todolist</p>")
}

async fn todos_handler() -> impl IntoResponse {
    Json(Todo::new(1, "first task"))
}

async fn create_todos_handler() -> impl IntoResponse {
    Json(StatusMessage::new("Success", StatusCode::OK.as_str()))
}

async fn login_handler(Json(login): Json<LoginRequest>) -> Json<LoginResponse> {
    let claims = Claims::new(1, "moni");
    let key = jwt::EncodingKey::from_secret(SECRET);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(LoginResponse::new(&token))
}

async fn app() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/login", post(login_handler))
        .route("/todos", get(todos_handler).post(create_todos_handler))
}

#[tokio::main]
async fn main() {
    let app = app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();
    axum::serve(listener, app).await.unwrap()

}


