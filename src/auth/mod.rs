use rocket::fairing::AdHoc;
mod database;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Posts", |rocket| async {
        rocket.mount("/posts/", routes![])
    })
}
