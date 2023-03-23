use rocket::FromForm;

#[derive(Debug, FromForm)]
pub struct WebMetion {
	source: String,
	target: String,
}
