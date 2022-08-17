use markdown_parser::read_file;
use rocket::serde::json::serde_json::{json,Value};

#[get("/<id>")]
pub fn render_post(id: String) -> Value {
    let md = read_file(format!("posts/{id}.md")).expect("");
    let content = md.content();
    json!({content:content})
}
