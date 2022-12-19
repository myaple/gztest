use crate::schema::geometries;
use diesel::prelude::*;
use geozero::wkb::Ewkb;

#[derive(Queryable, Debug, Insertable)]
#[diesel(table_name = geometries)]
pub struct Geom {
    pub name: String,
    pub geom: Option<Ewkb>,
}
