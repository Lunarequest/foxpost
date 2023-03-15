use rocket::fairing::AdHoc;
use routes::{login, login_get, logout};
mod database;
pub mod forms;
mod routes;

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("Users", |rocket| async {
		rocket.mount("/users/", routes![login, logout, login_get])
	})
}
