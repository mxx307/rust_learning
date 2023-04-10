use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};


pub async fn return_201() -> Response {
    (StatusCode::CREATED, "This is a  201".to_owned()).into_response()
}