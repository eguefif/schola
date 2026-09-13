use crate::db::curriculum_model::Curriculum;
use rusqlite::{params, Connection};

pub fn insert_all_curriculum(
    conn: &Connection,
    curriculum: Vec<Curriculum>,
) -> Result<(), Box<dyn std::error::Error>> {
    let query =
        "INSERT INTO curriculum (year, subject, grade, strand, goal) VALUES (?, ?, ?, ?, ?)";

    let mut stmt = conn.prepare(&query)?;
    curriculum.iter().for_each(|entry| {
        stmt.execute(params![
            entry.year,
            entry.subject,
            entry.grade,
            entry.strand,
            entry.goal
        ])
        .expect("Error while inserting curriculum row");
    });

    Ok(())
}
