#[derive(serde::Serialize, serde::Deserialize)]
pub struct Curriculum {
    pub year: u16,
    pub subject: String,
    pub grade: String,
    pub strand: String,
    pub goal: String,
}

impl Curriculum {
    pub fn new(year: u16, subject: String, grade: String, strand: String, goal: String) -> Self {
        Self {
            year,
            subject,
            grade,
            strand,
            goal,
        }
    }

    pub fn to_values() -> String {
        "(?, ?, ?, ?, ?)".to_string()
    }

    pub fn to_prepared_statement_values(&self) -> Vec<String> {
        return vec![
            self.year.to_string(),
            self.subject.clone(),
            self.grade.clone(),
            self.strand.clone(),
            self.goal.clone(),
        ];
    }
}
