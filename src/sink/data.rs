use std::net::{IpAddr, Ipv4Addr};
use rocket::Request;
use rocket::http::HeaderMap;
use rocket::request::{FromRequest, Outcome};

pub struct SinkRecord {
    pub(crate) data: [(&'static str, String); 3],
}

#[inline]
pub fn headermap_to_serde(headers: &HeaderMap) -> serde_json::Result<String> {
    serde_json::to_string(
        &serde_json::Map::from_iter(
            headers.iter().map(
                |item| (item.name.to_string(),
                        serde_json::Value::String(item.value.to_string()))
            ))
    )
}

use chrono::prelude::*;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SinkRecord {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let ip_key: &'static str = "ip";
        let time_key: &'static str = "time";
        let data_key: &'static str = "data";

        let ip = request.client_ip().unwrap_or(IpAddr::from(Ipv4Addr::new(0, 0, 0, 0)));

        Outcome::Success(SinkRecord {
            data: [
                (ip_key, ip.to_string()),
                (time_key, Utc::now().to_rfc3339().to_string()),
                (data_key, headermap_to_serde(request.headers()).unwrap_or_default()),
            ]
        })
    }
}

