use crate::db::get_conn;
use std::error;

pub fn init_db() -> Result<(), Box<dyn error::Error>> {
    let conn = get_conn()?;
    if !conn.table_exists(None, "year_plans")? {
        conn.execute(
            "CREATE TABLE year_plans (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            grade TEXT NOT NULL
        )",
            (),
        )?;
    }

    Ok(())
}
