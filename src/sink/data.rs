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
        const IP_KEY: &'static str = "ip";
        const TIME_KEY: &'static str = "time";
        const DATA_KEY: &'static str = "data";
        const IP_DEFAULT: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

        let ip = request.client_ip().unwrap_or(IpAddr::from(IP_DEFAULT));

        Outcome::Success(SinkRecord {
            data: [
                (IP_KEY, ip.to_string()),
                (TIME_KEY, Utc::now().to_rfc3339().to_string()),
                (DATA_KEY, headermap_to_serde(request.headers()).unwrap_or_default()),
            ]
        })
    }
}

