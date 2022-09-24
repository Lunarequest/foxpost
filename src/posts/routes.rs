use super::database::{NewPost, Post, UpdatePost};
use crate::auth::forms::Session;
use crate::db::BlogDBConn;
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::schema::posts as Posts;
use rocket::serde::json::{json, Json, Value};
use rocket_dyn_templates::{context, Template};

#[get("/<slug>")]
pub async fn render_post(db: BlogDBConn, slug: String) -> Option<Json<Post>> {
    db.run(move |conn| Posts::table.filter(Posts::slug.eq(slug)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[get("/new")]
pub async fn editor(sess: Session) -> Template {
    Template::render("editor", context! {title:"new post",sess:sess})
}

#[get("/json")]
pub async fn posts(db: BlogDBConn) -> Result<Json<Vec<Post>>, String> {
    match db
        .run(move |conn| {
            Posts::table
                .filter(Posts::draft.eq(true))
                .limit(5)
                .load::<Post>(conn)
        })
        .await
    {
        Ok(posts) => Ok(Json(posts)),
        Err(e) => Err(format!("{e}")),
    }
}

#[post("/new", data = "<post>")]
pub async fn new_post(db: BlogDBConn, sess: Session, post: Json<NewPost>) -> Result<Value, Value> {
    if !sess.isadmin {
        Err(json!({"errors":"you musted be logged in as an admin"}))
    } else {
        let post_value = post.clone();
        let post = Post::new(
            post_value.title.clone(),
            post_value.description.clone(),
            post_value.content.clone(),
            post_value.draft,
            sess.user,
        );
        let slug = post.slug.clone();
        match db
            .run(move |conn| {
                diesel::insert_into(Posts::table)
                    .values(post.clone())
                    .execute(conn)
            })
            .await
        {
            Err(_) => {
                Err(json!({"Errors":"a error occrued while trying to insert into the database"}))
            }
            Ok(_) => Ok(json!({ "slug": slug })),
        }
    }
}

#[post("/<slug>/update", data = "<post>")]
pub async fn update_post(
    db: BlogDBConn,
    sess: Session,
    post: Json<UpdatePost>,
    slug: String,
) -> Result<(), Value> {
    let mut posts: Post = match db
        .run(move |conn| Posts::table.filter(Posts::slug.eq(slug)).first(conn))
        .await
    {
        Ok(post) => post,
        Err(_e) => return Err(json!("missing post")),
    };
    if sess.user != posts.author || !sess.isadmin {
        return Err(json!({"errors":"you can not edit a post you didn't create"}));
    }
    posts = posts.clone().update(
        post.title.clone(),
        post.description.clone(),
        post.content.clone(),
    );
    match db
        .run(move |conn| {
            diesel::insert_into(Posts::table)
                .values(&posts)
                .execute(conn)
        })
        .await
    {
        Err(_) => Err(json!({"Errors":"a error occrued while trying to insert into the database"})),
        Ok(_) => Ok(()),
    }
}
