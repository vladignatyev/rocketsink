#[macro_use]
extern crate rocket;

use deadpool_redis::{Config, redis::cmd};
use deadpool_redis::Pool;
use rocket::http::Status;
use rocket::response::status;
use rocket_db_pools::{Database, deadpool_redis};

#[derive(Database)]
#[database("streamdb")]
struct StreamDb(Pool);

#[post("/<stream_name>")]
async fn sink_route(stream_db: &StreamDb, stream_name: &str) -> status::Custom<&'static str> {
    let items: [(&str, &str); 2] = [
        ("headers", ""),
        ("request", "")
    ];

    let conn = stream_db.get().await;
    if conn.is_err() {
        return status::Custom(Status::InternalServerError, "No DB connection.");
    }

    match cmd("XADD")
        .arg(stream_name)
        .arg("*")
        .arg(&items)
        .query_async::<_, ()>(&mut conn.unwrap())
        .await {
        Ok(_) => status::Custom(Status::Created, ""),
        Err(_) => status::Custom(Status::InternalServerError, "Write failed.")
    }
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(StreamDb::init())
        .mount("/sink", routes![sink_route])
        .launch()
        .await?;
    Ok(())
}