use rocket::fairing::AdHoc;
use routes::{new_post, posts, render_post, update_post};
mod database;
mod routes;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Posts", |rocket| async {
        rocket.mount(
            "/api/posts/",
            routes![posts, render_post, new_post, update_post],
        )
    })
}
