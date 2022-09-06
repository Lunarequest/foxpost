#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use posts::database::Post;
use rocket::fs::{relative, NamedFile};
use rocket_dyn_templates::{context, Template};
use schema::posts as Posts;
use std::path::{Path, PathBuf};
mod auth;
mod db;
mod posts;
mod schema;

#[get("/static/js/<asset>")]
async fn js(asset: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("static/js")).join(asset);
    if path.is_dir() {
        return None;
    }
    NamedFile::open(path).await.ok()
}

#[get("/static/css/<asset>")]
async fn css(asset: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("static/css")).join(asset);
    if path.is_dir() {
        return None;
    }
    NamedFile::open(path).await.ok()
}

#[get("/static/images/<asset>")]
async fn images(asset: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("static/images")).join(asset);
    if path.is_dir() {
        return None;
    }
    NamedFile::open(path).await.ok()
}

#[get("/")]
async fn index(db: db::BlogDBConn) -> Template {
    let posts = match db
        .run(move |conn| {
            Posts::table
                .filter(Posts::draft.eq(true))
                .limit(5)
                .load::<Post>(conn)
        })
        .await
    {
        Ok(posts) => Some(posts),
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
                                            posts:posts
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, images, js, css])
        .attach(Template::fairing())
        .attach(db::BlogDBConn::fairing())
        .attach(posts::stage())
        .attach(auth::stage())
}
