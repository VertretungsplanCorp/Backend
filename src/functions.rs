// %% INCLUDES %%
// % intern %
use crate::database::*;

// % extern %
use axum::{
    body::Body,
    extract::{Query, State},
    http,
    response::{Json, Response},
};
use chrono::{DateTime, Utc};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// %% HELPER FUNCTIONS %%

type BackendError = Response;
type BackendResponse = Result<Json<Value>, BackendError>;

pub async fn query_db<F, R>(pool: Pool, f: F) -> Result<R, BackendError>
where
    F: FnOnce(&mut PgConnection) -> Result<R, diesel::result::Error> + Send + 'static,
    R: Send + 'static,
{
    let m = pool
        .get()
        .await
        .map_err(|_| http::Response::new(Body::from(String::from("cannot access database"))))?;

    let r = m
        .interact(f)
        .await
        .map_err(|_| http::Response::new(Body::from(String::from("cannot query database"))))?
        .map_err(|_| {
            http::Response::new(Body::from(String::from("cannot interact with database")))
        })?;

    Ok(r)
}

// %% FUNCTIONS %%
pub async fn ping() -> &'static str {
    "Hallo aus dem Backend!"
}

pub async fn ping_json() -> Json<Value> {
    Json(json!({ "ping": 42 }))
}

impl From<models::Vertretung> for vp_api::Vertretung {
    fn from(
        models::Vertretung {
            stunde,
            fach,
            fach_neu,
            lehrer,
            lehrer_neu,
            raum_neu,
            raum,
            text,
            ..
        }: models::Vertretung,
    ) -> Self {
        Self {
            stunde: stunde as u8,
            fach,
            fach_neu,
            lehrer,
            lehrer_neu,
            raum,
            raum_neu,
            text,
        }
    }
}

struct Klasse {
    klasse: char,
    stufe: u8,
    vertretungen: Vec<models::Vertretung>,
}
impl From<Klasse> for vp_api::Klasse {
    fn from(
        Klasse {
            klasse,
            stufe,
            vertretungen,
        }: Klasse,
    ) -> Self {
        let mut m: HashMap<DateTime<Utc>, Vec<vp_api::Vertretung>> = HashMap::new();
        for e in vertretungen {
            if let Some(d) = m.get_mut(&e.datum) {
                d.push(e.into());
            } else {
                m.insert(e.datum, vec![e.into()]);
            }
        }

        let mut dati: Vec<vp_api::Datum> = Vec::new();
        for (datum, vertretungen) in m {
            dati.push(vp_api::Datum {
                datum: Some(datum),
                vertretungen,
            });
        }

        Self {
            klasse,
            stufe,
            dati,
            erstelldatum: Some(Utc::now()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct KlasseQuery {
    klasse: char,
    stufe: u8,
}
pub async fn get_klasse(
    Query(KlasseQuery { klasse, stufe }): Query<KlasseQuery>,
    State(pool): State<Pool>,
) -> BackendResponse {
    use schema::vertretungen::dsl as s;

    let vertretungen: Vec<models::Vertretung> = query_db(pool, move |c| {
        s::vertretungen
            .filter(s::stufe.eq(stufe as i16))
            .filter(s::klasse.eq(klasse.to_string()))
            .load(c)
    })
    .await?;

    let r: vp_api::Klasse = Klasse {
        klasse,
        stufe,
        vertretungen,
    }
    .into();

    Ok(Json(serde_json::to_value(r).unwrap()))
}

struct Stufe {
    stufe: u8,
    vertretungen: Vec<models::Vertretung>,
}
impl From<Stufe> for vp_api::Stufe {
    fn from(
        Stufe {
            stufe,
            vertretungen,
        }: Stufe,
    ) -> Self {
        let mut m: HashMap<char, Vec<models::Vertretung>> = HashMap::new();
        for e in vertretungen {
            let k = e.klasse.clone().pop().unwrap();
            if let Some(v) = m.get_mut(&k) {
                v.push(e);
            } else {
                m.insert(k, vec![e]);
            }
        }

        let mut klassen: Vec<vp_api::Klasse> = Vec::new();
        for (klasse, vertretungen) in m {
            klassen.push(
                Klasse {
                    klasse,
                    stufe,
                    vertretungen,
                }
                .into(),
            );
        }

        Self { stufe, klassen }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StufeQuery {
    stufe: u8,
}
pub async fn get_stufe(
    Query(StufeQuery { stufe }): Query<StufeQuery>,
    State(pool): State<Pool>,
) -> BackendResponse {
    use schema::vertretungen::dsl as s;

    let vertretungen: Vec<models::Vertretung> = query_db(pool, move |c| {
        s::vertretungen.filter(s::stufe.eq(stufe as i16)).load(c)
    })
    .await?;

    let r: vp_api::Stufe = Stufe {
        stufe,
        vertretungen,
    }
    .into();

    Ok(Json(serde_json::to_value(r).unwrap()))
}

pub async fn get_info(State(pool): State<Pool>) -> BackendResponse {
    use schema::infos::dsl::*;
    let r: models::Infos = query_db(pool, move |c| infos.first(c)).await?;

    Ok(Json(
        serde_json::to_value(vp_api::Info {
            datum: Some(r.datum),
            text: r.text,
            erstelldatum: Some(r.erstelldatum),
        })
        .unwrap(),
    ))
}
