use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct SignUp {
    pub username: String,
    pub email: String,
    pub passwd1: String,
    pub passwd2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub username: String,
    pub passwd: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Session {
    pub authkey: String,
    pub user: String,
    pub timestamp: i64,
}
