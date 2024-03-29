use private::{delete_entry, drafts, edit, editor, get_content, new_post, update_post};
use public::{posts, render_post, search_by_tag};
use rocket::fairing::AdHoc;
pub mod database;
mod json;
mod private;
mod public;

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("Posts", |rocket| async {
		rocket
			.mount(
				"/api/posts/",
				routes![posts, new_post, update_post, get_content],
			)
			.mount(
				"/posts",
				routes![
					editor,
					render_post,
					drafts,
					edit,
					search_by_tag,
					delete_entry
				],
			)
	})
}
