use std::net::IpAddr;

use rocket::http::{HeaderMap, Status};
use rocket_db_pools::deadpool_redis::redis::cmd;

use crate::sink::data::RequestHeaders;
use crate::sink::db::StreamDb;
use crate::sink::error::SinkError;

#[post("/<stream_name>")]
pub async fn sink_route(stream_db: &StreamDb,
                    stream_name: &str,
                    request_headers: RequestHeaders,
                    ipaddr: IpAddr) -> Result<Status, SinkError> {
    let items: [(&str, String); 2] = [
        ("headers", request_headers.json.to_string()),
        ("ip", ipaddr.to_string())
    ];

    let mut conn = stream_db.get().await?;

    cmd("XADD")
        .arg(stream_name)
        .arg("*")
        .arg(&items)
        .query_async::<_, ()>(&mut conn)
        .await?;

    Ok(Status::Ok)
}