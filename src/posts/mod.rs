use rocket::fairing::AdHoc;
use routes::{drafts, edit, editor, new_post, posts, render_post, update_post};
pub mod database;
mod routes;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Posts", |rocket| async {
        rocket
            .mount("/api/posts/", routes![posts, new_post, update_post])
            .mount("/posts", routes![editor, render_post, drafts, edit])
    })
}
