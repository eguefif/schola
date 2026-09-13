use crate::db::model_error::ModelError;

#[derive(serde::Serialize)]
pub struct YearPlan {
    id: i64,
    name: String,
    grade: String,
    is_valid: bool,
    errors: Option<Vec<ModelError>>,
}

const GRADES: [&str; 8] = ["ps", "ms", "gs", "cp", "ce1", "ce2", "cm1", "cm2"];

impl YearPlan {
    pub fn new(id: i64, name: String, grade: String) -> Self {
        let mut errors = vec![];

        if GRADES.iter().any(|item| item == &grade) {
            errors.push(ModelError::new(
                "grade".to_string(),
                "Not a valid grade".to_string(),
            ));
        }

        if name.len() == 0 {
            errors.push(ModelError::new(
                "name".to_string(),
                "Name is required".to_string(),
            ));
        }
        Self {
            id,
            name,
            grade,
            is_valid: true,
            errors: None,
        }
    }

    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self::new(row.get(0)?, row.get(1)?, row.get(2)?))
    }
}
