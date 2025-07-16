use super::schema::*;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::serialize::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// #[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize, Debug, Clone, PartialEq)]
// #[diesel(table_name = vertretungen)]
#[derive(Serialize, Deserialize)]
pub struct Vertretung {
    pub id: Uuid,
    pub klasse: String,
    pub stunde: i16,
    pub fach: String,
    pub fach_neu: Option<String>,
    pub raum: Option<String>,
    pub raum_neu: Option<String>,
    pub text: Option<String>,
    pub datum: DateTime<Utc>,
    pub erstelldatum: DateTime<Utc>,
}

// #[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize, Debug, Clone, PartialEq)]
// #[diesel(table_name = infos)]
#[derive(Serialize, Deserialize)]
pub struct Infos {
    pub id: Uuid,
    pub text: String,
    pub datum: DateTime<Utc>,
    pub erstelldatum: DateTime<Utc>,
}
