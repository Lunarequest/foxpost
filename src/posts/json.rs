use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
pub struct JsonEntry {
    pub id: u32,
    pub href: String,
    pub title: String,
    pub date: String,
    pub body: Option<String>,
}
