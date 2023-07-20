use super::{database::Post, json::JsonEntry};
use crate::{config::Config, db::BlogDBConn, schema::posts as Posts};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{ExpressionMethods, PgArrayExpressionMethods, QueryDsl, RunQueryDsl};
use pulldown_cmark::{html, Options, Parser};
use rocket::{http::Status, serde::json::Json, State};
use rocket_dyn_templates::{context, Template};

fn render_to_html(markdown: String) -> String {
	let mut options = Options::all();
	options.insert(Options::all());
	let parser = Parser::new_ext(&markdown, options);
	let mut html_output = String::new();
	html::push_html(&mut html_output, parser);
	html_output
}

fn convert(timestamp: i64) -> String {
	let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap_or_default();
	let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
	datetime.format("%Y-%m-%d").to_string()
}

#[get("/tag/<tag>")]
pub async fn search_by_tag(
	db: BlogDBConn,
	tag: String,
	config: &State<Config>,
) -> Result<Template, (Status, String)> {
	let tag_cloned = tag.clone();
	let mut posts: Vec<Post> = match db
		.run(move |conn| {
			Posts::table
				.filter(Posts::draft.eq(false))
				.filter(Posts::tags.contains(vec![Some(tag_cloned)]))
				.load::<Post>(conn)
		})
		.await
	{
		Ok(posts) => posts,
		Err(e) => return Err((Status::InternalServerError, format!("{e}"))),
	};
	posts.reverse();
	Ok(Template::render(
		"tags",
		context! {
			title: format!("all posts with tag {}", tag),
			config: &config.other,
			posts: posts
		},
	))
}

#[get("/<slug>")]
pub async fn render_post(db: BlogDBConn, slug: String, config: &State<Config>) -> Option<Template> {
	let post: Post = match db
		.run(move |conn| {
			Posts::table
				.filter(Posts::draft.eq(false))
				.filter(Posts::slug.eq(slug))
				.first(conn)
		})
		.await
	{
		Ok(post) => post,
		Err(e) => {
			eprintln!("{e}");
			return None;
		}
	};
	if post.draft {
		return None;
	}
	println!("{:#?}", post);
	let content = render_to_html(post.clone().content.unwrap_or_default());
	Some(Template::render(
		"post",
		context! {
			title: post.title.clone(),
			content:content,
			post: post,
			domain: &config.domain,
			config: &config.other
		},
	))
}

#[get("/json")]
pub async fn posts(db: BlogDBConn) -> Result<Json<Vec<JsonEntry>>, (Status, String)> {
	match db
		.run(move |conn| {
			Posts::table
				.filter(Posts::draft.eq(false))
				.limit(5)
				.load::<Post>(conn)
		})
		.await
	{
		Ok(posts) => {
			let mut search_posts: Vec<JsonEntry> = vec![];
			for (count, post) in (32..).zip(posts.into_iter()) {
				let entry = JsonEntry {
					id: count,
					href: format!("/posts/{}", post.slug),
					title: post.title,
					date: convert(post.published),
					body: post.description,
				};
				search_posts.append(&mut vec![entry]);
			}
			Ok(Json(search_posts))
		}
		Err(e) => Err((Status::InternalServerError, format!("{e}"))),
	}
}
