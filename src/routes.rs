use super::db::BlogDBConn;
use super::{auth::forms::Session, posts::database::Post, schema::posts as Posts};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{fs::NamedFile, request::FlashMessage};
use rocket_dyn_templates::{context, Template};
use std::path::{Path, PathBuf};

#[get("/search")]
pub async fn search() -> Template {
	Template::render("search", context! {title:"search"})
}

#[get("/favicon.ico")]
pub async fn favicon() -> Option<NamedFile> {
	NamedFile::open("./static/images/favicon.ico").await.ok()
}

#[get("/about")]
pub async fn about() -> Template {
	Template::render("about", context! { title: "about"})
}

#[get("/static/<type>/<asset>")]
pub async fn static_files(r#type: String, asset: PathBuf) -> Option<NamedFile> {
	match r#type.as_str() {
		"css" => {
			let path = Path::new("./static/css").join(asset);
			if path.is_dir() {
				return None;
			}
			NamedFile::open(path).await.ok()
		}
		"js" => {
			let path = Path::new("./static/js").join(asset);
			if path.is_dir() {
				return None;
			}
			NamedFile::open(path).await.ok()
		}
		"images" => {
			let path = Path::new("./static/images").join(asset);
			if path.is_dir() {
				return None;
			}
			NamedFile::open(path).await.ok()
		}
		_ => None,
	}
}

#[get("/")]
pub async fn index(
	db: BlogDBConn,
	flash: Option<FlashMessage<'_>>,
	sess: Option<Session>,
) -> Template {
	let posts = match db
		.run(move |conn| {
			Posts::table
				.filter(Posts::draft.eq(false))
				.order_by(Posts::published)
				.load::<Post>(conn)
		})
		.await
	{
		Ok(posts) => {
			let mut posts = posts;
			//for some reason order by returns
			// small->large
			//we want large->small
			posts.reverse();
			Some(posts)
		}
		Err(e) => {
			//if there are error's we will know
			eprintln!("{e}");
			None
		}
	};

	Template::render(
		"index",
		context! {
			title:"Home",
			posts:posts,
			flash:flash,
			sess:sess,
		},
	)
}
