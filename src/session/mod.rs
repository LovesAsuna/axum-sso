use std::fmt::{Debug, Formatter};
use async_session::{Session, SessionStore};
use axum::async_trait;
use redis::Commands;

pub(crate) static COOKIE_NAME: &str = "SESSION";

pub struct RedisSession(pub redis::Client);

impl Debug for RedisSession {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Clone for RedisSession {
    fn clone(&self) -> Self {
        RedisSession(self.0.clone())
    }
}

#[async_trait]
impl SessionStore for RedisSession {
    async fn load_session(&self, session_id: String) -> async_session::Result<Option<Session>> {
        let mut conn = self.0.get_connection()?;
        let value = conn.get::<String, String>(session_id)?;
        let session = serde_json::from_str::<Session>(&value)?;
        Ok(Some(session))
    }

    async fn store_session(&self, session: Session) -> async_session::Result<Option<String>> {
        let mut conn = self.0.get_connection()?;
        let value = serde_json::to_string(&session)?;
        let _ = conn.set::<&str, &str, ()>(session.id(), &value)?;
        Ok(Some(value))
    }

    async fn destroy_session(&self, session: Session) -> async_session::Result {
        let mut conn = self.0.get_connection()?;
        Ok(conn.del(session.id())?)
    }

    async fn clear_store(&self) -> async_session::Result {
        Ok(())
    }
}