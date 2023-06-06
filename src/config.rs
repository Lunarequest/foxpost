use rocket::serde::json::Value;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Config {
	pub domain: String,
	#[serde(flatten)]
	pub other: HashMap<String, Value>,
}
