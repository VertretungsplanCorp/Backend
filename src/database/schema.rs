// @generated automatically by Diesel CLI.

diesel::table! {
    infos (id) {
        id -> Uuid,
        text -> Text,
        datum -> Timestamptz,
        erstelldatum -> Timestamptz,
    }
}

diesel::table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::table! {
    vertretungen (id) {
        id -> Uuid,
        #[max_length = 1]
        klasse -> Bpchar,
        stufe -> Int2,
        stunde -> Int2,
        #[max_length = 20]
        fach -> Nullable<Varchar>,
        #[max_length = 20]
        fach_neu -> Nullable<Varchar>,
        #[max_length = 3]
        raum -> Nullable<Varchar>,
        #[max_length = 3]
        raum_neu -> Nullable<Varchar>,
        lehrer -> Nullable<Varchar>,
        lehrer_neu -> Nullable<Varchar>,
        #[max_length = 100]
        text -> Nullable<Varchar>,
        datum -> Timestamptz,
        erstelldatum -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(infos, spatial_ref_sys, vertretungen,);
