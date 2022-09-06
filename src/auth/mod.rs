use rocket::fairing::AdHoc;
use routes::{login, logout, signup, signup_page};
mod database;
pub mod forms;
mod routes;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Users", |rocket| async {
        rocket.mount("/users/", routes![signup, login, logout, signup_page])
    })
}
