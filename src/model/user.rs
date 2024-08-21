use crate::error::AuthRedirect;
use crate::session;
use crate::session::RedisSession;
use async_session::SessionStore;
use axum::extract::{FromRef, FromRequestParts};
use axum::{async_trait, RequestPartsExt};
use axum_extra::typed_header::TypedHeaderRejectionReason;
use axum_extra::{headers, TypedHeader};
use http::header;
use http::request::Parts;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "name", enum_name = "Name")]
    pub name: String,
    #[sea_orm(column = "password")]
    pub password: String,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[async_trait]
impl<S> FromRequestParts<S> for Model
where
    S: Send + Sync,
    redis::Client: FromRef<S>,
    DatabaseConnection: FromRef<S>
{
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("unexpected error getting cookies: {e}"),
            })?;
        let session_id = cookies.get(session::COOKIE_NAME).ok_or(AuthRedirect)?;
        let session_store = RedisSession(redis::Client::from_ref(state));
        let session = session_store.load_session(session_id.to_string()).await.map_err(|_| AuthRedirect)?.ok_or(AuthRedirect)?;

        let user_id = session.get::<String>("user_id").ok_or(AuthRedirect)?;

        let db = DbConn::from_ref(state);
        Ok(Entity::find_by_id(user_id.parse().unwrap_or(0)).one(&db).await.map_err(|_| AuthRedirect)?.ok_or(AuthRedirect)?)
    }
}