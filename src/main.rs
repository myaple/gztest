use diesel::pg::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::Connection;
//use diesel::prelude::*;

use crate::models::*;

use geozero::wkb::Ewkb;
use geozero::ToGeo;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("set database url");
    PgConnection::establish(&database_url).unwrap()
}

fn main() {
    use self::schema::geometries;
    let conn = &mut establish_connection();

    let results = geometries::dsl::geometries
        .limit(10)
        .load::<Geom>(conn)
        .expect("error loading geoms");

    for item in results {
        println!("{}", item.name);
        println!("-------");
        println!("{:#?}", item.geom);
    }

//    let wkb = Ewkb(vec![
//        1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 64, 0, 0, 0, 0, 0, 0, 52, 192,
//    ]);
//
//    let newgeometry = Geom {
//        name: "testonetwo".to_string(),
//        geom: Some(wkb),
//    };
//
//    let res: Geom = diesel::insert_into(geometries::table)
//        .values(&newgeometry)
//        .get_result(conn)
//        .expect("brokebroke");
//
//    println!("------------");
//    println!("{:#?}", res);
//
//    println!("------------");
//
//    println!("{:#?}", res.geom.unwrap().to_geo().unwrap());
}
