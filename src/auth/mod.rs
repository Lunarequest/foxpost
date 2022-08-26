use rocket::fairing::AdHoc;
use routes::{login, signup};
mod database;
mod forms;
mod routes;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Users", |rocket| async {
        rocket.mount("/users/", routes![signup, login])
    })
}
