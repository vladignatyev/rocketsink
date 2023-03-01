use deadpool_redis::Pool;
use rocket_db_pools::deadpool_redis;

pub use rocket_db_pools::Database;

#[derive(Database)]
#[database("redis")]
pub struct StreamDb(Pool);
