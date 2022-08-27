use super::database::{NewPost, Post};
use crate::auth::forms::Session;
use crate::db::BlogDBConn;
use crate::diesel::RunQueryDsl;
use crate::schema::posts as Posts;
use markdown_parser::read_file;
use rocket::{
    fs::NamedFile,
    response::content::RawJson,
    serde::json::{json, Json, Value},
};

#[get("/<id>")]
pub fn render_post(id: String) -> Value {
    let md = read_file(format!("posts/{id}.md")).expect("");
    let content = md.content();
    json!({ content: content })
}

#[get("/json")]
pub async fn posts() -> RawJson<NamedFile> {
    let file = NamedFile::open("posts/posts.json")
        .await
        .expect("unable to open file");
    RawJson(file)
}

#[post("/new", data = "<post>")]
pub async fn new_post(
    db: BlogDBConn,
    sess: Session,
    post: Json<NewPost>,
) -> Result<(), RawJson<&'static str>> {
    if !sess.isadmin {
        let errors = "{\"errors\":\"you musted be logged in as an admin\"}";
        Err(RawJson(errors))
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
            Err(_) => Err(RawJson("{\"errors\":\"unable to inster into db\"}")),
            Ok(_) => Ok(()),
        }
    }
}
