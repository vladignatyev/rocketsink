use rocket::{Request, Response};
use rocket::http::ext::IntoCollection;
use rocket::request::{FromRequest, Outcome};
use rocket::response::{Responder, status};
use rocket::serde::{Serialize, Serializer};

pub struct RequestHeaders {
    pub json: String,
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
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

