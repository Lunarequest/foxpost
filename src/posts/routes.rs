use markdown_parser::read_file;
use rocket::{
    serde::json::serde_json::{json, Value},
    fs::NamedFile,
    response::content::RawJson
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