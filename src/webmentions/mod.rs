mod forms;
mod routes;
use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("Posts", |rocket| async {
		rocket.mount("/webmentions", routes![])
	})
}
