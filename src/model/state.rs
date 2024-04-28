use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
#[derive(Clone)]
pub struct AppState {
    pub redis: redis::Client,
    pub db: DatabaseConnection,
}

impl FromRef<AppState> for redis::Client {
    fn from_ref(state: &AppState) -> Self {
        state.redis.clone()
    }
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}