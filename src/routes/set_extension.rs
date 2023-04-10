use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

use super::hello_world::Msg;

pub async fn set_extension<B>(mut request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let msg = headers.get("msg").ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let msg = msg.to_str().map_err(|_error| StatusCode::BAD_REQUEST)?.to_owned();
    let extensions = request.extensions_mut();
    extensions.insert(Msg(msg));
    Ok(next.run(request).await)
}