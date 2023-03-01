use rocket::Request;
use rocket::http::HeaderMap;
use rocket::request::{FromRequest, Outcome};

pub struct RequestHeaders {
    pub json: String,
}

#[inline]
pub fn headermap_to_json_string(headers: &HeaderMap) -> serde_json::Result<String> {
    let g = serde_json::Map::from_iter(
        headers.iter().map(
            |item| (item.name.to_string(),
                    serde_json::Value::String(item.value.to_string()))
        ));
    serde_json::to_string(&g)
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RequestHeaders {
            json: headermap_to_json_string(request.headers()).unwrap()
        })
    }
}

