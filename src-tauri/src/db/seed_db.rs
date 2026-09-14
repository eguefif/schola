use crate::db::curriculum::insert_all_curriculum;
use crate::db::curriculum_model::Curriculum;
use chrono::NaiveDate;
use rusqlite::Connection;
use tauri_plugin_http::reqwest;

const CURRICULUM_URL: &str = "http://localhost:4000/api/curriculum";
const YEAR_URL: &str = "http://localhost:4000/api/year";

pub async fn fetch_curriculum() -> Result<Vec<Curriculum>, Box<dyn std::error::Error>> {
    log::info!("Fetching curriculum");
    let response = reqwest::get(CURRICULUM_URL).await?;
    let text_content = response.text().await?;
    let content = serde_json::from_str::<Vec<Curriculum>>(&text_content)?;
    log::info!("Deserialized data successfully");
    Ok(content)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SchoolPeriod {
    start: NaiveDate,
    end: NaiveDate,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SchoolZone {
    periode1: SchoolPeriod,
    periode2: SchoolPeriod,
    periode3: SchoolPeriod,
    periode4: SchoolPeriod,
    periode5: SchoolPeriod,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SchoolCalendar {
    a: SchoolZone,
    b: SchoolZone,
    c: SchoolZone,
}

pub async fn fetch_year() -> Result<SchoolCalendar, Box<dyn std::error::Error>> {
    log::info!("Fetching year schedule");
    let response = reqwest::get(YEAR_URL).await?;
    let text_content = response.text().await?;
    let content = serde_json::from_str::<SchoolCalendar>(&text_content)?;
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
