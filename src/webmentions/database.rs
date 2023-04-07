use diesel::Queryable;
use serde::Serialize;

#[derive(Debug, Queryable)]
pub struct CompletedRequest {
	pub id: i32,
	pub source: String,
	pub target: String,
	pub content: String,
	pub author: String,
	pub author_url: String,
	pub url: String,
}
