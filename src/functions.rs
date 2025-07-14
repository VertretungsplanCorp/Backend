// %% INCLUDES %%
use axum::{extract::State, response::Json};
use deadpool_diesel::postgres::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use serde_json::{json, Value};
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::task;
use tokio::time::{sleep, Duration};
use tokio::{main, net::TcpSocket};
use tower_http::cors::*;

// %% FUNCTIONS %%
pub async fn ping() -> &'static str {
    "Hallo aus dem Backend!"
}

pub async fn ping_json() -> Json<Value> {
    Json(json!({ "ping": 42 }))
}

pub async fn get_klasse(klasse: String, State(pool): State<Pool>) -> Json<Value> {
    // let m: Manager = pool.get().await.unwrap();
    // m.interact(move |c| insert_into(ratings))
    todo!()
}
