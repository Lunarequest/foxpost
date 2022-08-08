use rocket::fairing::AdHoc;
mod json;
use json::posts;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Posts", |rocket| async {
        rocket.mount("/posts/", routes![posts])
    })
}
