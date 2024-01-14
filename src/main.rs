use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let listener = TcpListener::bind("127.0.0.1")
        .await
        .expect("bind localhost error");
    axum::serve(listener, app())
        .await
        .expect("axum server error");
}

fn app() -> Router {
    Router::new().route("/", get(|| async { "hello world" }))
}
