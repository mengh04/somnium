#[derive(sqlx::FromRow, Debug)]
pub struct Song {
    pub id: i64,
    pub title: String,
    pub artist: Option<String>,
    pub path: String,
}
