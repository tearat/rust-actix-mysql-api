use serde::Serialize;

#[derive(Serialize)]
pub struct Hero {
    pub id: u64,
    pub title: String,
}

pub struct HeroExtended {
    pub id: u64,
    pub title: String,
    pub body: String
}