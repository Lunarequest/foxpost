use rocket::fairing::AdHoc;
use routes::{login, logout};
mod database;
pub mod forms;
mod routes;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Users", |rocket| async {
        rocket.mount("/users/", routes![login, logout, routes::signup, routes::signup_page])
    })
}
