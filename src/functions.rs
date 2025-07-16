// %% INCLUDES %%
use crate::database::models::*;
use axum::{
    extract::{Query, State},
    response::Json,
};
use deadpool_diesel::postgres::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::task;
use tokio::time::{sleep, Duration};
use tokio::{main, net::TcpSocket};
use tower_http::cors::*;
use uuid::Uuid;

// %% FUNCTIONS %%
pub async fn ping() -> &'static str {
    "Hallo aus dem Backend!"
}

pub async fn ping_json() -> Json<Value> {
    Json(json!({ "ping": 42 }))
}

#[derive(Serialize, Deserialize)]
pub struct KlasseQuery {
    klasse: String,
}
pub async fn get_klasse(
    Query(klasse_query): Query<KlasseQuery>,
    State(pool): State<Pool>,
) -> Json<Value> {
    // let m: Manager = pool.get().await.unwrap();
    // m.interact(move |c| insert_into(ratings))
    // let v = Vertretung {
    //     id: Uuid::default(),
    //     klasse: "8c".into(),
    //     fach: "E".into(),
    //     fach_neu: Some("D".into()),
    //     raum: Some("H101".into()),
    //     raum_neu: Some("H309".into()),
    //     text: Some("Arbeitsauftrag auf Mebis".into()),
    //     datum: chrono::Local::now().into(),
    //     stunde: 1,
    //     erstelldatum: chrono::Local::now().into(),
    // };

    let r = vp_api::KlassenVertretung {
        klasse: "12e".into(),
        dati: vec![vp_api::Datum {
            datum: chrono::Local::now().to_rfc2822(),
            vertretungen: vec![vp_api::Vertretung {
                stunde: 1,
                fach: Some("E".into()),
                raum: Some("H301".into()),
                text: Some("AB auf Mebis".into()),
                lehrer: Some("smi".into()),
                raum_neu: Some("H302".into()),
                lehrer_neu: Some("cla".into()),
                fach_neu: Some("D".into()),
            }],
        }],
        erstellt_am: chrono::Local::now().to_rfc2822(),
    };

    Json(serde_json::to_value(r).unwrap())
}
