use rocket::fairing::AdHoc;
use routes::{editor, new_post, posts, render_post, update_post};
pub mod database;
mod routes;

//TODO: Finish implemention of editor using https://github.com/Ionaru/easy-markdown-editor
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Posts", |rocket| async {
        rocket
            .mount("/api/posts/", routes![posts, new_post, update_post])
            .mount("/posts", routes![editor, render_post])
    })
}
