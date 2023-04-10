use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MyJson {
    message: String,
}

#[derive(Serialize)]
pub struct MyJsonResponse {
    message: String,
    meesage_from_server: String,
}

pub async fn upload_file(Json(body): Json<MyJson>) -> Json<MyJsonResponse> {
    Json(MyJsonResponse { message: body.message, meesage_from_server: "from by mxx".to_owned() })
}