mod forms;
mod routes;
use rocket::fairing::AdHoc;
use routes::recive_mention;
mod database;

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("webmentions", |rocket| async {
		rocket.mount("/webmentions", routes![recive_mention])
	})
}
