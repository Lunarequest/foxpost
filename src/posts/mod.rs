use rocket::fairing::AdHoc;
use routes::{
	drafts, edit, editor, get_content, new_post, posts, render_post, search_by_tag, update_post,
};
pub mod database;
mod json;
mod routes;

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("Posts", |rocket| async {
		rocket
			.mount(
				"/api/posts/",
				routes![posts, new_post, update_post, get_content],
			)
			.mount(
				"/posts",
				routes![editor, render_post, drafts, edit, search_by_tag],
			)
	})
}
