use redis::Connection;
use crate::helpers::config;

pub fn get_connection() -> redis::RedisResult<Connection> {
    let client = redis::Client::open(config::get("REDIS_URL"))?;
    client.get_connection()
}
