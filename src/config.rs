use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Config {
	pub domain: String,
}
