use rocket::fairing::AdHoc;
use routes::{posts,render_post};
mod routes;
mod database;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Posts", |rocket| async {
        rocket.mount("/api/posts/", routes![posts, render_post])
    })
}
