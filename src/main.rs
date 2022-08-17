#[macro_use]
extern crate rocket;
use posts::stage;
mod posts;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(stage())
}
