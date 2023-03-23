use rocket::form::Form;

use super::forms::WebMetion;

#[post("/", data = "<mention>")]
async fn recive_mention(mention: Form<WebMetion>) -> Result<(), ()> {
	Ok(())
}
