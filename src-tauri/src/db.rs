pub mod init_db;
pub mod year_plan;
pub mod year_plan_model;

use rusqlite::Connection;
use std::error;

pub fn get_conn() -> Result<Connection, Box<dyn error::Error>> {
    return Connection::open("./data.db").map_err(Into::into);
}
