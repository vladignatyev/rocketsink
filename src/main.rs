extern crate redis;
#[macro_use]
extern crate rocket;

use redis::{Commands, RedisResult};
use rocket::response::status;
use rocket::http::Status;
use rocket::State;

#[get("/")]
fn stat_route(redis: &State<RedisConnection>) -> &'static str {
    "Hello, world!"
}

#[post("/<key>")]
fn sink_route(key: &str, redis: &State<RedisConnection>) -> String {
    match redis.conn.get_connection() {
        Ok(mut c) => {
            let items: [(&str, &str); 2] = [
                ("headers", ""),
                ("request", "")
            ];

            match c.xadd::<&str, &str, &str, &str, String>(key, "*", &items) {
                Ok(_) => {
                    // Status::Ok
                    "OKay".to_string()
                },
                Err(e) => {
                    // Status::InternalServerError(format!("{}",e))
                    format!("{}", e)
                }
            }
        }
        Err(e) => {
            // Status::InternalServerError
            format!("{}", e)
        }
    }
}


struct RedisConnection {
    pub conn: redis::Client,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let redis_url = "redis://localhost:6379/0";

    let redis_conn = redis::Client::open(redis_url)
        .expect("Unable connect to Redis");

    let _rocket = rocket::build()
        .manage(RedisConnection { conn: redis_conn })
        .mount("/sink", routes![sink_route])
        // .mount("/stat", routes![stat_route])
        .launch()
        .await?;
    Ok(())
}