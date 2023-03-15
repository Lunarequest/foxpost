use rocket::{fairing::AdHoc, http::Status, Request};
use rocket_dyn_templates::{context, Template};

#[catch(404)]
async fn not_found(_req: &Request<'_>) -> Template {
	Template::render("404", context! {title: "this page doesn't exist"})
}

#[catch(505)]
async fn internal_error(_: &Request<'_>) -> Template {
	Template::render("505", context! {title: "Oops we made a mistake"})
}

#[catch(403)]
async fn naught_baka(_: &Request<'_>) -> Template {
	Template::render("403", context! {title: "You've been naughty baka"})
}

#[catch(default)]
async fn default_catcher(status: Status, _req: &Request<'_>) -> Template {
	Template::render(
		"error",
		context! {
			title: "Unkown error",
			error: status.code,
		},
	)
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("catchers", |rocket| async {
		rocket.register(
			"/",
			catchers![not_found, default_catcher, internal_error, naught_baka],
		)
	})
}
