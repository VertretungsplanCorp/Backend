use axum::{extract::State, response::Json, routing::get, Router};
use deadpool_diesel::{
    postgres::{Manager, Pool},
    Runtime::Tokio1,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use serde_json::{json, Value};
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::{main, net::TcpSocket};
use tower_http::cors::*;

mod database;
use database::schema::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("database/migrations");

#[main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ############################## DATABASE ############################## //

    let database_url = env::var("DATABASE_URL").expect("Please provide a correct DATABASE_URL.");

    let manager = Manager::new(database_url, Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

    {
        let manager = pool.get().await.unwrap();
        manager
            .interact(|c| c.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    let app = Router::new()
        .route("/ping", get(ping))
        .layer(cors)
        .with_state(pool);

    let host = env::var("INTERN_SV_HOST")
        .expect("An intern server host must be specified")
        .parse::<Ipv4Addr>()
        .expect("Please provide an intern ipv4 adress as host");
    let port = env::var("INTERN_SV_PORT")
        .expect("An intern server port must be specified")
        .parse::<u16>()
        .expect("Please provide an integer as intern port");

    let addr = SocketAddr::new(IpAddr::V4(host), port);

    println!("Adress: {addr}");

    let socket = TcpSocket::new_v4().expect("Could not crate tcp socket");
    socket
        .set_reuseaddr(true)
        .expect("Could not set feature to reuse adress of tcp socket");
    socket
        .bind(addr)
        .expect("Could not add adress to tcp socket");

    let listener = socket.listen(10).expect("Could not establish tcp listener");

    println!("Serving...");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Could not serve");
}

async fn ping() -> &'static str {
    "Hallo aus dem Backend!"
}

async fn getKlasse(klasse: String, State(pool): State<Pool>) -> Json<Value> {
    // let m: Manager = pool.get().await.unwrap();
    // m.interact(move |c| insert_into(ratings))
    todo!()
}
