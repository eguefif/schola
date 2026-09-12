#[derive(serde::Serialize)]
pub struct YearPlan {
    id: i64,
    name: String,
    grade: String,
}

impl YearPlan {
    pub fn new(id: i64, name: String, grade: String) -> Self {
        Self { id, name, grade }
    }

    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self::new(row.get(0)?, row.get(1)?, row.get(2)?))
    }
}
