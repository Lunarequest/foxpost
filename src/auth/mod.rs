use rocket::fairing::AdHoc;
use routes::{login, logout, signup};
mod database;
pub mod forms;
mod routes;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Users", |rocket| async {
        rocket.mount("/api/users/", routes![signup, login, logout])
    })
}
