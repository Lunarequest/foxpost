use crate::schema::pending_requests;
use diesel::Insertable;
use rocket::FromForm;
use url::Url;

#[derive(Debug, Clone, FromForm, Insertable)]
#[diesel(table_name = pending_requests)]
pub struct WebMetion {
	source: String,
	target: String,
}

impl WebMetion {
	pub fn verify(self: Self) -> bool {
		match Url::parse(&self.source) {
			Err(e) => {
				eprintln!("{e}");
				false
			}
			Ok(_) => match Url::parse(&self.target) {
				Err(e) => {
					eprintln!("{e}");
					false
				}
				Ok(_) => true,
			},
		}
	}
}
