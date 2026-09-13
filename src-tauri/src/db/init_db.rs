use rusqlite::Connection;

pub fn init_db(conn: &Connection) {
    init_year_plan(conn);
    init_curriculum(conn);
}

fn init_year_plan(conn: &Connection) {
    if !conn
        .table_exists(None, "year_plans")
        .expect("Impossible to check if table exists")
    {
        match conn.execute(
            "CREATE TABLE year_plans (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            grade TEXT NOT NULL
        )",
            (),
        ) {
            Ok(_) => log::info!("Sucessfully created year_plans table"),
            Err(err) => panic!("Error while creating year_plans table {}", err),
        }
    };
}

fn init_curriculum(conn: &Connection) {
    if !conn
        .table_exists(None, "curriculum")
        .expect("Impossible to check if table exists")
    {
        match conn.execute(
            "CREATE TABLE curriculum (
            id INTEGER PRIMARY KEY,
            year INTEGER NOT NULL,
            subject TEXT NOT NULL,
            grade TEXT NOT NULL,
            strand TEXT NOT NULL,
            goal TEXT NOT NULL
        )",
            (),
        ) {
            Ok(_) => log::info!("Sucessfully created curriculum table"),
            Err(err) => panic!("Error while creating curriculum table {}", err),
        }
    };
}
