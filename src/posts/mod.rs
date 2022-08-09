use rocket::fairing::AdHoc;
mod json;
mod post;
use json::posts;
use post::render_post;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Posts", |rocket| async {
        rocket.mount("/posts/", routes![posts, render_post])
    })
}
