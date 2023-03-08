use super::database::{NewPost, Post, Tag};
use super::json::JsonEntry;
use crate::auth::forms::Session;
use crate::db::BlogDBConn;
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::schema::posts as Posts;
use crate::schema::tags as Tags;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::PgArrayExpressionMethods;
use diesel::pg::upsert::excluded;
use pulldown_cmark::{html, Options, Parser};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_dyn_templates::{context, Template};

fn render_to_html(markdown: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::all());
    let parser = Parser::new_ext(&markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn convert(timestamp: i64) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap_or_default();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime.to_string()
}

#[get("/tag/<tag>")]
pub async fn search_by_tag(db: BlogDBConn, tag: String) -> Result<Template, (Status, String)> {
    let posts = match db
        .run(move |conn| {
            Posts::table
                .filter(Posts::tags.contains(vec![Some(tag)]))
                .execute(conn)
        })
        .await
    {
        Ok(posts) => posts,
        Err(e) => return Err((Status::InternalServerError, format!("{e}"))),
    };
    Ok(Template::render(
        "tags",
        context! {
            posts: posts
        },
    ))
}

#[get("/edit/<slug>")]
pub async fn edit(
    db: BlogDBConn,
    sess: Session,
    slug: String,
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
pub async fn drafts(db: BlogDBConn, sess: Session) -> Result<Template, (Status, String)> {
    if sess.isadmin {
        let mut posts: Vec<Post> = match db.run(move |conn| Posts::table.load(conn)).await {
            Ok(posts) => posts,
            Err(e) => return Err((Status::UnprocessableEntity, e.to_string())),
        };
        //newest first
        posts.reverse();
        Ok(Template::render(
            "all_posts",
            context! {
                title: "All",
                posts:posts
            },
        ))
    } else {
        Err((
            Status::Unauthorized,
            String::from("you are not authorised to view this page"),
        ))
    }
}

#[get("/<slug>")]
pub async fn get_content(db: BlogDBConn, slug: String) -> String {
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
}

#[get("/<slug>")]
pub async fn render_post(db: BlogDBConn, slug: String) -> Option<Template> {
    let post: Post = match db
        .run(move |conn| Posts::table.filter(Posts::slug.eq(slug)).first(conn))
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
    let content = render_to_html(post.clone().content.unwrap_or_default());
    Some(Template::render(
        "post",
        context! {
            title: post.title.clone(),
            content:content,
            post: post
        },
    ))
}

#[get("/new")]
pub async fn editor(sess: Session) -> Result<Template, Status> {
    if sess.isadmin {
        Ok(Template::render(
            "editor",
            context! {title:"new post",sess:sess},
        ))
    } else {
        Err(Status::Unauthorized)
    }
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
            post_value.title.clone(),
            post_value.description.clone(),
            post_value.content.clone(),
            post_value.draft,
            tags,
            sess.user,
        );
        let slug = post.slug.clone();
        match db
            .run(move |conn| diesel::insert_into(Posts::table).values(post).execute(conn))
            .await
        {
            Err(_) => Err((
                Status::InternalServerError,
                json!({"Errors":"a error occrued while trying to insert into the database"}),
            )),
            Ok(_) => {
                match db
                    .run(move |conn| {
                        diesel::insert_into(Tags::table)
                            .values(&tag_insert)
                            .execute(conn)
                    })
                    .await
                {
                    Ok(_) => Ok(json!({ "slug": slug })),
                    Err(_) => Err((
                        Status::InternalServerError,
                        json!({"Errors":"a error occrued while trying to insert into the database"}),
                    )),
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
    //over write tags in posts and update tags db
    match db
        .run(move |conn| {
            diesel::update(Posts::table.find(slugval))
                .set((
                    Posts::dsl::draft.eq(post.draft),
                    Posts::dsl::title.eq(post.title.clone()),
                    Posts::dsl::description.eq(post.description.clone()),
                    Posts::dsl::content.eq(post.content.clone()),
                    Posts::dsl::tags.eq(tags),
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
                        .values(tag_insert)
                        .on_conflict(Tags::tag)
                        .do_update()
                        .set(Tags::tag.eq(excluded(Tags::tag)))
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
