use super::database::{now, NewPost, Post, Tag};

use crate::auth::forms::Session;
use crate::config::Config;
use crate::db::BlogDBConn;
use crate::diesel::{delete, ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::schema::posts as Posts;
use crate::schema::tags as Tags;

use rocket::response::Redirect;
use rocket::{
	http::Status,
	serde::json::{json, Json, Value},
	State,
};
use rocket_dyn_templates::{context, Template};

#[get("/edit/<slug>")]
pub async fn edit(
	db: BlogDBConn,
	sess: Session,
	slug: String,
	config: &State<Config>,
) -> Result<Template, (Status, String)> {
	if sess.isadmin {
		match db
			.run(move |conn| {
				Posts::table
					.filter(Posts::slug.eq(slug))
					.first::<Post>(conn)
			})
			.await
		{
			Err(e) => Err((Status::UnprocessableEntity, e.to_string())),
			Ok(post) => Ok(Template::render(
				"edit_post",
				context! {
					title: format!("edit {}",post.title),
					post: post,
					config: &config.other
				},
			)),
		}
	} else {
		Err((
			Status::Unauthorized,
			String::from("you are not authorised to view this page"),
		))
	}
}

#[get("/all")]
pub async fn drafts(
	db: BlogDBConn,
	sess: Session,
	config: &State<Config>,
) -> Result<Template, (Status, String)> {
	if sess.isadmin {
		let mut posts: Vec<Post> = match db
			.run(move |conn| Posts::table.order_by(Posts::published).load(conn))
			.await
		{
			Ok(posts) => posts,
			Err(e) => return Err((Status::UnprocessableEntity, e.to_string())),
		};
		//newest first
		posts.reverse();
		Ok(Template::render(
			"all_posts",
			context! {
				title: "All",
				posts:posts,
				sess: sess,
				lax: true,
				config: &config.other,
			},
		))
	} else {
		Err((
			Status::Unauthorized,
			String::from("you are not authorised to view this page"),
		))
	}
}

#[get("/delete/<slug>")]
pub async fn delete_entry(
	db: BlogDBConn,
	sess: Session,
	slug: String,
) -> Result<Redirect, (Status, String)> {
	if sess.isadmin {
		match db
			.run(move |conn| delete(Posts::table.filter(Posts::slug.eq(slug))).execute(conn))
			.await
		{
			Ok(_) => Ok(Redirect::to(uri!("/posts/all"))),
			Err(e) => Err((Status::InternalServerError, format!("{e}"))),
		}
	} else {
		Err((
			Status::Unauthorized,
			String::from("you are not authorised to view this page"),
		))
	}
}

#[get("/<slug>")]
pub async fn get_content(db: BlogDBConn, slug: String, sess: Session) -> String {
	if sess.isadmin {
		let post: Post = match db
			.run(move |conn| Posts::table.filter(Posts::slug.eq(slug)).first(conn))
			.await
		{
			Ok(post) => post,
			Err(e) => {
				eprintln!("{e}");
				return String::new();
			}
		};
		match post.content {
			Some(content) => content,
			None => String::new(),
		}
	} else {
		slug
	}
}

#[get("/new")]
pub async fn editor(sess: Session, config: &State<Config>) -> Result<Template, Status> {
	if sess.isadmin {
		Ok(Template::render(
			"editor",
			context! {title:"new post",sess:sess, config: &config.other},
		))
	} else {
		Err(Status::Unauthorized)
	}
}

#[post("/new", data = "<post>")]
pub async fn new_post(
	db: BlogDBConn,
	sess: Session,
	post: Json<NewPost>,
) -> Result<Value, (Status, Value)> {
	if !sess.isadmin {
		Err((
			Status::Unauthorized,
			json!({"errors":"you musted be logged in as an admin"}),
		))
	} else {
		let post_value = post.clone();
		let tags_split = post_value.tags.split(", ");
		let mut tags: Vec<Option<String>> = vec![];
		let mut tag_insert: Vec<Tag> = vec![];
		for tag in tags_split {
			tags.append(&mut vec![Some(tag.to_string())]);
			tag_insert.append(&mut vec![Tag {
				tag: tag.to_string(),
			}]);
		}
		let post = Post::new(
			&post_value.title,
			&post_value.description,
			&post_value.content,
			post_value.draft,
			tags,
			sess.user,
			&post_value.noteid,
		);

		let slug = post.slug.to_owned();
		match db
			.run(move |conn| diesel::insert_into(Posts::table).values(post).execute(conn))
			.await
		{
			Err(e) => {
				eprintln!("{e}");
				Err((
					Status::InternalServerError,
					json!({"Errors":"a error occrued while trying to insert into the database",
								"Debug": format!("{e}")
					}),
				))
			}
			Ok(_) => {
				match db
					.run(move |conn| {
						diesel::insert_into(Tags::table)
							.values(&tag_insert)
							.on_conflict_do_nothing()
							.execute(conn)
					})
					.await
				{
					Ok(_) => Ok(json!({ "slug": slug })),
					Err(e) => {
						eprintln!("{e}");
						Err((
							Status::InternalServerError,
							json!({"Errors":"a error occrued while trying to insert into the database"}),
						))
					}
				}
			}
		}
	}
}

#[post("/<slug>/update", data = "<post>")]
pub async fn update_post(
	db: BlogDBConn,
	sess: Session,
	post: Json<NewPost>,
	slug: String,
) -> Result<Value, (Status, Value)> {
	let slugval = slug.clone();
	let posts: Post = match db
		.run(move |conn| Posts::table.filter(Posts::slug.eq(slug)).first(conn))
		.await
	{
		Ok(post) => post,
		Err(e) => {
			eprintln!("{e}");
			return Err((Status::NotFound, json!({"Errors":"missing post"})));
		}
	};
	if sess.user != posts.author || !sess.isadmin {
		return Err((
			Status::Unauthorized,
			json!({"Errors":"you can not edit a post you didn't create"}),
		));
	}
	let mut tags: Vec<Option<String>> = vec![];
	let mut tag_insert: Vec<Tag> = vec![];
	let tags_split = post.tags.split(", ");
	for tag in tags_split {
		tags.append(&mut vec![Some(tag.to_string())]);
		tag_insert.append(&mut vec![Tag {
			tag: tag.to_string(),
		}]);
	}
	if posts.draft {
		match db
			.run(move |conn| {
				diesel::update(Posts::table.find(slugval))
					.set((
						Posts::dsl::draft.eq(post.draft),
						Posts::dsl::title.eq(post.title.clone()),
						Posts::dsl::description.eq(post.description.clone()),
						Posts::dsl::content.eq(post.content.clone()),
						Posts::dsl::tags.eq(tags),
						Posts::dsl::noteid.eq(post.noteid.clone()),
						Posts::dsl::published.eq(now()),
					))
					.execute(conn)
			})
			.await
		{
			Err(e) => {
				eprintln!("{e}");
				Err((
					Status::InternalServerError,
					json!({"status":"error","Errors":"a error occrued while trying to insert into the database"}),
				))
			}
			Ok(_) => {
				match db
					.run(move |conn| {
						diesel::insert_into(Tags::table)
							.values(&tag_insert)
							.on_conflict_do_nothing()
							.execute(conn)
					})
					.await
				{
					Ok(_) => Ok(json!({"status":"sucess"})),
					Err(e) => {
						eprintln!("{e}");
						Err((
							Status::InternalServerError,
							json!({"status":"error","Errors":"a error occrued while trying to insert into the database"}),
						))
					}
				}
			}
		}
	} else {
		match db
			.run(move |conn| {
				diesel::update(Posts::table.find(slugval))
					.set((
						Posts::dsl::draft.eq(post.draft),
						Posts::dsl::title.eq(post.title.clone()),
						Posts::dsl::description.eq(post.description.clone()),
						Posts::dsl::content.eq(post.content.clone()),
						Posts::dsl::tags.eq(tags),
						Posts::dsl::noteid.eq(post.noteid.clone()),
					))
					.execute(conn)
			})
			.await
		{
			Err(e) => {
				eprintln!("{e}");
				Err((
					Status::InternalServerError,
					json!({"status":"error","Errors":"a error occrued while trying to insert into the database"}),
				))
			}
			Ok(_) => {
				match db
					.run(move |conn| {
						diesel::insert_into(Tags::table)
							.values(&tag_insert)
							.on_conflict_do_nothing()
							.execute(conn)
					})
					.await
				{
					Ok(_) => Ok(json!({"status":"sucess"})),
					Err(e) => {
						eprintln!("{e}");
						Err((
							Status::InternalServerError,
							json!({"status":"error","Errors":"a error occrued while trying to insert into the database"}),
						))
					}
				}
			}
		}
	}

	//over write tags in posts and update tags db
}
