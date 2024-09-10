use std::error::Error;

use axum::routing::post;
use axum::Router;
use redis::Client;
use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;

use crate::model::state::AppState;

mod error;
mod middleware;
mod model;
mod router;
mod session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    let client = Client::open("redis://127.0.0.1/")?;
    let opt = ConnectOptions::new("postgresql://localhost");
    let db = Database::connect(opt).await?;
    let state = AppState { redis: client, db };

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("bind localhost error");
    let app = app()
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::check_session,
        ))
        .with_state(state);
    axum::serve(listener, app).await.expect("axum server error");
    Ok(())
}

fn app() -> Router<AppState> {
    Router::new().route("/login", post(router::login))
}
