use crate::rocket;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn index() {
	let client = Client::tracked(rocket()).expect("valid rocket instance");
	let response = client.get("/").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type().unwrap(), ContentType::HTML);
}

#[test]
fn static_ok() {
	let client = Client::tracked(rocket()).expect("valid rocket instance");
	let response = client.get("/static/js/search.js").dispatch();
	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.content_type().unwrap(), ContentType::JavaScript);
}

#[test]
fn static_missing() {
	let client = Client::tracked(rocket()).expect("valid rocket instance");
	let response = client.get("/static/js/this_does_not_exist.js").dispatch();
	assert_eq!(response.status(), Status::NotFound);
	assert_eq!(response.content_type().unwrap(), ContentType::HTML);
}
