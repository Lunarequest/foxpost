#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel_migrations::embed_migrations;
use rocket::{fairing::AdHoc, Build, Rocket};
use rocket_dyn_templates::tera::{from_value, to_value, Error, Value};
use rocket_dyn_templates::{Engines, Template};
use std::collections::HashMap;
mod auth;
mod db;
mod errors;
mod image;
mod posts;
mod routes;
mod schema;
mod xml;

async fn run_migrations_fairing(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("migrations");
    println!("running migrations");
    let conn = db::BlogDBConn::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
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
        .mount(
            "/",
            routes![
                routes::index,
                routes::static_files,
                routes::search,
                routes::about,
                routes::favicon
            ],
        )
        .attach(Template::custom(|engines: &mut Engines| {
            engines.tera.register_function("convert", convert)
        }))
        .attach(errors::stage())
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
