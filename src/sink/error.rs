use deadpool;
use deadpool::managed::PoolError;
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;
use rocket_db_pools::deadpool_redis::redis::RedisError;

#[derive(Debug)]
pub struct SinkError {
    message: String,
}

impl From<PoolError<RedisError>> for SinkError {
    fn from(value: PoolError<RedisError>) -> Self {
        Self {
            message: value.to_string()
        }
    }
}

impl From<RedisError> for SinkError {
    fn from(value: RedisError) -> Self {
        Self {
            message: value.to_string()
        }
    }
}

impl<'a> Responder<'a, 'a> for SinkError {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'a> {
        Ok(
            Response::build()
                .status(Status::InternalServerError)
                .raw_header("X-Error-Message", self.message)
                .finalize()
        )
    }
}