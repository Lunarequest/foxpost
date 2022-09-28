use rocket::fairing::AdHoc;
use routes::{image_form, media, upload};
mod routes;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("images", |rocket| async {
        rocket.mount("/media", routes![upload, image_form, media])
    })
}
