#[macro_use]
extern crate rocket;
use posts::stage;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};
use std::path::{Path, PathBuf};
mod posts;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/assets/<file>")]
async fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets").join(file)).await.ok()
}
#[get("/assets/css/<file>")]
async fn css(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/css").join(file))
        .await
        .ok()
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/favicon.ico")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, assets, favicon, css])
        .attach(Template::fairing())
        .attach(stage())
}
