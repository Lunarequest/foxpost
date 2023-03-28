use super::forms::WebMetion;
use crate::{db::BlogDBConn, schema::pending_requests};
use diesel::{insert_into, RunQueryDsl};
use rocket::{form::Form, http::Status};

#[post("/", data = "<mention>")]
pub async fn recive_mention(conn: BlogDBConn, mention: Form<WebMetion>) -> Result<Status, Status> {
	let mention_value = mention.clone();
	let check_value = mention.clone();
	if check_value.verify() {
		match conn
			.run(|c| {
				insert_into(pending_requests::table)
					.values(mention_value)
					.execute(c)
			})
			.await
		{
			Ok(_) => return Ok(Status::Accepted),
			Err(e) => {
				eprintln!("{e}");
				return Err(Status::UnprocessableEntity);
			}
		}
	}
	Err(Status::UnprocessableEntity)
}
