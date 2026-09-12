use crate::db::get_conn;
use crate::db::year_plan_model::YearPlan;
use std::error;

pub fn create_year_plan(name: String, grade: String) -> Result<YearPlan, Box<dyn error::Error>> {
    let conn = get_conn()?;
    match conn.execute(
        "INSERT INTO year_plans (name, grade) VALUES (?, ?)",
        (&name, &grade),
    ) {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            return Ok(YearPlan::new(id, name, grade));
        }
        Err(err) => return Err(Box::new(err)),
    }
}

pub fn list_all() -> Result<Vec<YearPlan>, Box<dyn error::Error>> {
    let conn = get_conn()?;
    let mut stmt = conn.prepare("SELECT * from year_plans")?;
    let plans = stmt
        .query_map([], YearPlan::from_row)?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(plans)
}
