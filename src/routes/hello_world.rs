use axum::{Extension, Json, TypedHeader};
use axum::extract::Query;
use axum::headers::UserAgent;
use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::routes::ShareData;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryParams {
    message: String,
    id: i32,
}

#[derive(Clone)]
pub struct Msg(pub String);


#[derive(Serialize)]
pub struct JsonResponse {
    message: String,
    id: i32,
    user_agent: String,
    x_message: String,
    share_data: String,
    msg: String,
}

pub async fn hello_world(Query(query): Query<QueryParams>,
                         TypedHeader(user_agent): TypedHeader<UserAgent>,
                         headers: HeaderMap,
                         Extension(shared_data): Extension<ShareData>,
                         Extension(msg): Extension<Msg>)
                         -> Json<JsonResponse> {
    let x_msg = headers.get("x-message").unwrap();
    let x_msg = x_msg.to_str().unwrap().to_owned();
    let shareddata = shared_data.message;
    Json(JsonResponse {
        message: query.message,
        id: query.id,
        user_agent: user_agent.to_string(),
        x_message: x_msg,
        share_data: shareddata,
        msg: msg.0,
    })
}