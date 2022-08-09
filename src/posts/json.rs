use rocket::{fs::NamedFile, response::content::RawJson};

#[get("/json")]
pub async fn posts() -> RawJson<NamedFile> {
    let file = NamedFile::open("posts/posts.json")
        .await
        .expect("unable to open file");
    RawJson(file)
}
