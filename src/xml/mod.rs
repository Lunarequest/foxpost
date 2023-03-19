use crate::{config::Config, db::BlogDBConn, posts::database::Post, schema::posts as Posts};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{fairing::AdHoc, State};
use rocket_dyn_templates::{context, Template};

#[get("/sitemap.xml")]
async fn sitemap(db: BlogDBConn, config: &State<Config>) -> Template {
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
		"sitemap",
		context! {
			host: &config.domain,
			posts:posts
		},
	)
}

#[get("/index.xml")]
async fn rss(db: BlogDBConn, config: &State<Config>) -> Template {
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
		"rss",
		context! {
			host: &config.domain,
			posts:posts
		},
	)
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("xml", |rocket| async {
		rocket.mount("/", routes![sitemap, rss])
	})
}
