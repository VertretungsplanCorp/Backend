use super::schema::*;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[diesel(table_name = vertretungen)]
pub struct Vertretung {
    pub id: Uuid,
    pub klasse: String,
    pub stufe: i16,
    pub stunde: i16,
    pub fach: Option<String>,
    pub fach_neu: Option<String>,
    pub raum: Option<String>,
    pub raum_neu: Option<String>,
    pub lehrer: Option<String>,
    pub lehrer_neu: Option<String>,
    pub text: Option<String>,
    pub datum: DateTime<Utc>,
    pub erstelldatum: DateTime<Utc>,
}

#[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[diesel(table_name = infos)]
pub struct Infos {
    pub id: Uuid,
    pub text: String,
    pub datum: DateTime<Utc>,
    pub erstelldatum: DateTime<Utc>,
}
