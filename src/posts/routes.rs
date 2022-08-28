use super::database::{NewPost, Post};
use crate::auth::forms::Session;
use crate::db::BlogDBConn;
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::schema::posts as Posts;
use rocket::serde::json::{json, Json, Value};

#[get("/<slug>")]
pub async fn render_post(db: BlogDBConn, slug: String) -> Option<Json<Post>> {
    db.run(move |conn| Posts::table.filter(Posts::slug.eq(slug)).first(conn))
        .await
        .map(Json)
        .ok()
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
        Ok(posts) => return Ok(Json(posts)),
        Err(e) => return Err(format!("{e}")),
    }
}

#[post("/new", data = "<post>")]
pub async fn new_post(db: BlogDBConn, sess: Session, post: Json<NewPost>) -> Result<(), Value> {
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
        match db
            .run(move |conn| {
                diesel::insert_into(Posts::table)
                    .values(&post)
                    .execute(conn)
            })
            .await
        {
            Err(_) => {
                Err(json!({"Errors":"a error occrued while trying to insert into the database"}))
            }
            Ok(_) => Ok(()),
        }
    }
}
