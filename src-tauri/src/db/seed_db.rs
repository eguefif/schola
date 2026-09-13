use crate::db::curriculum::insert_all_curriculum;
use crate::db::curriculum_model::Curriculum;
use rusqlite::Connection;
use tauri_plugin_http::reqwest;

const URL: &str = "http://localhost:4000/api/curriculum";

pub async fn fetch_curriculum() -> Result<Vec<Curriculum>, Box<dyn std::error::Error>> {
    log::info!("Fetching curriculum");
    let response = reqwest::get(URL).await?;
    log::info!("Result: {}", response.status());
    let text_content = response.text().await?;
    let content = serde_json::from_str::<Vec<Curriculum>>(&text_content)?;
    log::info!("Deserialized data successfully");
    Ok(content)
}

pub fn seed_db(conn: &Connection, content: Vec<Curriculum>) {
    log::info!("Seeding curriculum");
    match insert_all_curriculum(conn, content) {
        Ok(_) => log::info!("Sucessfully seeding curriculum"),
        Err(err) => {
            log::error!("Error while seeding curriculum {}", err);
            panic!("Error while seeding curriculum {}", err);
        }
    }
}
