
use axum::{Extension, middleware, Router, routing::{get, post}};
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

use hello_world::hello_world;
use path_variables::{hard_code_path, path_variables};
use upload_file::upload_file;
use set_extension::set_extension;
use always_error::always_error;
use return_201::return_201;

mod hello_world;
mod upload_file;
mod path_variables;
mod set_extension;
mod always_error;
mod return_201;

#[derive(Clone)]
pub struct ShareData {
    pub message: String,
}


pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = ShareData { message: "This is a message from shared data".to_owned() };

    Router::new()
        .route("/", get(hello_world))
        .route_layer(middleware::from_fn(set_extension))
        .route("/upload_file", post(upload_file))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hard_code_path))
        .layer(cors)
        .layer(Extension(shared_data))
        .route("/always_error", get(always_error))
        .route("/return_201", get(return_201))
}