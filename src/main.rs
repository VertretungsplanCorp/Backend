use axum::{routing::get, Router};
use tokio::{main, net::TcpListener};

#[main]
async fn main() {
    let app = Router::new().route("/ping", get(ping));
    // Router //
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ping() -> &'static str {
    "Hallo aus dem Backend!"
}
