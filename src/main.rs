// %% INCLUDES %%
use axum::{routing::get, Router};
use chrono::Local;
use deadpool_diesel::{
    postgres::{Manager, Pool},
    Runtime::Tokio1,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use pyo3::prelude::*;
use std::ffi::CString;
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::net::TcpSocket;
use tokio::task;
use tokio::time::{sleep, Duration};
use tower_http::cors::*;

// %% MODULE %%
mod database;
use database::schema::*;

mod functions;
use functions::*;

// %% FUNCTIONS %%

// % main %
#[tokio::main]
async fn main() {
    let main_process = task::spawn(async { api().await });
    let python_scheduler = task::spawn(async { scraper().await });
    let _ = tokio::try_join!(main_process, python_scheduler).unwrap();
}

// % embedded migrations %
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("database/migrations");

// % SCRAPER %
async fn scraper() -> PyResult<()> {
    loop {
        let now = Local::now();
        let target_time = now
            .date_naive()
            .and_hms_opt(7, 3, 0)
            .unwrap()
            .and_local_timezone(now.timezone())
            .unwrap();

        let duration_until = if target_time < now {
            (target_time + chrono::Duration::days(1)) - now
        } else {
            target_time - now
        };

        println!(
            "Next Python execution in {} seconds",
            duration_until.num_seconds()
        );
        sleep(Duration::from_secs(duration_until.num_seconds() as u64)).await;

        Python::with_gil(|py| {
            println!("Executing Python script at 8:00");
            py.run(
                &CString::new(include_str!("scraper/scraping.py")).unwrap(),
                None,
                None,
            )
            .unwrap();
        });
    }
}

// % API %
async fn api() {
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
        .route("/ping_json", get(ping_json))
        .layer(cors)
        .with_state(pool);

    let host = env::var("SV_HOST")
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
