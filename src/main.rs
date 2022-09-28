#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
use auth::forms::Session;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel_migrations::embed_migrations;
use posts::database::Post;
use rocket::{
    fairing::AdHoc,
    fs::{relative, NamedFile},
    request::FlashMessage,
    Build, Rocket,
};
use rocket_dyn_templates::tera::{from_value, to_value, Error, Value};
use rocket_dyn_templates::{context, Engines, Template};
use schema::posts as Posts;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
mod auth;
mod db;
mod image;
mod posts;
mod schema;
mod xml;

#[get("/static/<type>/<asset>")]
async fn static_files(r#type: String, asset: PathBuf) -> Option<NamedFile> {
    match r#type.as_str() {
        "css" => {
            let path = Path::new(relative!("static/css")).join(asset);
            if path.is_dir() {
                return None;
            }
            NamedFile::open(path).await.ok()
        }
        "js" => {
            let path = Path::new(relative!("static/js")).join(asset);
            if path.is_dir() {
                return None;
            }
            NamedFile::open(path).await.ok()
        }
        "images" => {
            let path = Path::new(relative!("static/images")).join(asset);
            if path.is_dir() {
                return None;
            }
            NamedFile::open(path).await.ok()
        }
        "webfonts" => {
            let path = Path::new(relative!("static/webfonts")).join(asset);
            if path.is_dir() {
                return None;
            }
            NamedFile::open(path).await.ok()
        }
        _ => None,
    }
}

#[get("/")]
async fn index(
    db: db::BlogDBConn,
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

async fn run_migrations_fairing(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("migrations");

    let conn = db::BlogDBConn::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}

#[get("/search")]
async fn search() -> Template {
    Template::render("search", context! {title:"search"})
}

fn convert(args: &HashMap<String, Value>) -> Result<Value, Error> {
    #[allow(clippy::or_fun_call)]
    let timestamp = match from_value::<i64>(
        args.get("timestamp")
            .ok_or::<Error>("no timestamp".into())?
            .clone(),
    ) {
        Ok(time) => time,
        Err(_) => return Err("is the timestamp a int?".into()),
    };
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    match to_value(datetime.to_string()) {
        Ok(time) => Ok(time),
        Err(e) => Err(format!("{e}").into()),
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, static_files, search])
        .attach(Template::custom(|engines: &mut Engines| {
            engines.tera.register_function("convert", convert)
        }))
        .attach(db::BlogDBConn::fairing())
        .attach(AdHoc::on_ignite(
            "Diesel Migrations",
            run_migrations_fairing,
        ))
        .attach(posts::stage())
        .attach(auth::stage())
        .attach(image::stage())
        .attach(xml::stage())
}
