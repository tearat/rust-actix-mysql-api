use serde::Serialize;

#[derive(Serialize)]
pub struct Hero {
    pub id: u64,
    pub title: String
}