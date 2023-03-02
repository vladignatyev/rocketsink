use rocket::http::Status;
use rocket::post;
#[allow(unused_imports)]
use rocket_db_pools::deadpool_redis::redis::{AsyncCommands, cmd};

use crate::sink::data::SinkRecord;
use crate::sink::db::StreamDb;
use crate::sink::error::SinkError;

#[post("/<stream_name>")]
pub async fn sink_route(stream_db: &StreamDb,
                        sink_data: SinkRecord,
                        stream_name: &str,
) -> Result<Status, SinkError> {
    let mut conn = stream_db.get().await?;

    cmd("XADD")
        .arg(stream_name)
        .arg("*")
        .arg(&sink_data.data)
        .query_async::<_, ()>(&mut conn)
        .await?;

    Ok(Status::Ok)
}