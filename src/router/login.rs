use crate::error::{AppError, RegisterRedirect};
use crate::model::user;
use crate::model::user::Entity as User;
use crate::session;
use crate::session::RedisSession;
use anyhow::Context;
use async_session::{Session, SessionStore};
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use http::header::SET_COOKIE;
use http::HeaderMap;
use sea_orm::EntityTrait;
use sea_orm::{ColumnTrait, DatabaseConnection, QueryFilter};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(redis): State<redis::Client>,
    Query(user_name): Query<String>,
    Query(_): Query<String>,
) -> Result<Response, AppError> {
    let user = User::find().filter(user::Column::Name.eq(user_name)).one(&db).await?;
    if user.is_none() {
        return Ok(RegisterRedirect.into_response());
    }
    let user = user.unwrap();
    let mut session = Session::new();
    session
        .insert("user_id", user.id)
        .context("failed in inserting serialized value into session")?;
    let session_store = RedisSession(redis);
    // Store session and get corresponding cookie
    let cookie = session_store.store_session(session).await?;
    if cookie.is_none() {
        return Ok(RegisterRedirect.into_response());
    }
    let cookie = cookie.unwrap();
    // Build the cookie
    let cookie = format!("{}={cookie}; SameSite=Lax; Path=/", session::COOKIE_NAME);

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok(headers.into_response())
}
