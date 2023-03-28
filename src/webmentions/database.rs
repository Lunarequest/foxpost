use crate::schema::completed_requests;
use diesel::Queryable;

#[derive(Debug, Queryable)]
#[diesel(table_name=completed_requests)]
pub struct CompletedRequest {
	pub id: u64,
	pub source: String,
	pub target: String,
	pub content: String,
	pub author: String,
	pub pfp: String,
}
