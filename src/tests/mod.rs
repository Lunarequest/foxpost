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
