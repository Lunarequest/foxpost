#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
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
#[cfg(test)]
mod tests;
mod xml;

async fn run_migrations_fairing(rocket: Rocket<Build>) -> Rocket<Build> {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    println!("running migrations");
    db::BlogDBConn::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|c| {
            c.run_pending_migrations(MIGRATIONS)
                .expect("Diesel migrations");
        })
        .await;
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
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap_or_default();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    match to_value(datetime.to_string()) {
        Ok(time) => Ok(time),
        Err(e) => Err(format!("{e}").into()),
    }
}

fn tags_to_list(args: &HashMap<String, Value>) -> Result<Value, Error> {
    let tags = match from_value::<Vec<String>>(
        args.get("tags").ok_or::<Error>("No tags?".into())?.clone(),
    ) {
        Ok(tags) => tags,
        Err(e) => return Err(format!("{e}").into()),
    };
    let tag_string = tags.join(", ");
    match to_value(tag_string.to_string()) {
        Ok(tags) => Ok(tags),
        Err(e) => Err(format!("{e}").into()),
    }
}

#[launch]
fn rocket() -> _ {
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
            engines.tera.register_function("convert", convert);
            engines.tera.register_function("tags_to_list", tags_to_list);
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
