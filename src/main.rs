pub mod sink;

#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(sink::db::StreamDb::init())
        .mount("/sink", routes![sink::routes::sink_route])
        .launch()
        .await?;
    Ok(())
}