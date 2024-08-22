use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;

use crate::error::AuthRedirect;
use crate::model::user::Model;

pub async fn check_session(
    user: Option<Model>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AuthRedirect> {
    match user {
        Some(_) => Ok(next.run(request).await),
        None => Err(AuthRedirect),
    }
}
