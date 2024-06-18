use diesel::{prelude::*, mysql::MysqlConnection};
use crate::helpers::config;

pub fn establish_connection() -> MysqlConnection {
    let database_url = config::get("DATABASE_URL");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
