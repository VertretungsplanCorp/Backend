use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::{main, net::TcpSocket};

#[main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/ping", get(ping));

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
