#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
mod auth;
mod posts;
mod schema;
mod db;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(posts::stage()).attach(auth::stage())
}
