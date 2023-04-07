use super::{database::CompletedRequest, forms::WebMetion};
use crate::{
	db::BlogDBConn,
	schema::{completed_requests, pending_requests},
};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{form::Form, http::Status};
use serde_json::{json, Value};

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

#[get("/<slug>")]
pub async fn get_mentions(conn: BlogDBConn, slug: String) -> Result<Value, Status> {
	let mentions: Vec<CompletedRequest> = match conn
		.run(|c| {
			completed_requests::table
				.filter(completed_requests::target.eq(slug))
				.load::<CompletedRequest>(c)
		})
		.await
	{
		Ok(mentions) => mentions,
		Err(e) => {
			eprintln!("{e}");
			return Err(Status::InternalServerError);
		}
	};
	Ok(json!({ "mentions": mentions }))
}
