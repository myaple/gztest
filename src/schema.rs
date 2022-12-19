// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use geozero::postgis::diesel::sql_types::*;

    geometries (name) {
        name -> Varchar,
        geom -> Nullable<Geometry>,
    }
}
