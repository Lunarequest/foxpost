#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
mod auth;
mod db;
mod posts;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::BlogDBConn::fairing())
        .attach(posts::stage())
        .attach(auth::stage())
}
