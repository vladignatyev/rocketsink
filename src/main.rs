#[macro_use]
extern crate rocket;

use std::net::IpAddr;
use deadpool_redis::{Config, redis::cmd};
use deadpool_redis::Pool;
use rocket::http::{HeaderMap, Status};
use rocket::http::ext::IntoCollection;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status;
use rocket::serde::{Serialize, Serializer};
use rocket_db_pools::{Database, deadpool_redis};
// use rocket_db_pools::deadpool_redis::redis;

#[derive(Database)]
#[database("streamdb")]
struct StreamDb(Pool);

struct RequestHeaders {
    json: String,
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // let g = request.headers().into_collection().;
        let g = serde_json::Map::from_iter(
            request.headers().clone()
                .into_iter().map(|item| {
                (item.name.to_string(), serde_json::Value::String(item.value.to_string()))
            }));

        Outcome::Success(RequestHeaders {
            json: serde_json::to_string(&g).unwrap()
        })
    }
}

#[post("/<stream_name>")]
async fn sink_route(stream_db: &StreamDb, stream_name: &str,
                    request_headers: RequestHeaders, ipaddr: IpAddr) -> status::Custom<&'static str> {
    let items: [(&str, String); 2] = [
        ("headers", request_headers.json.to_string()),
        ("ip", ipaddr.to_string())
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
        Ok(_) => status::Custom(Status::Created, "Oka"),
        Err(_) => status::Custom(Status::InternalServerError, "Write failed.")
    }
    // status::Custom(Status::Created, "Oka")
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