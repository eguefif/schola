#[derive(serde::Serialize)]
pub struct ModelError {
    field: String,
    message: String,
}

impl ModelError {
    pub fn new(field: String, message: String) -> Self {
        Self { field, message }
    }
}
