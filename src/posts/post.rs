use markdown;
use markdown_parser::read_file;
use rocket_dyn_templates::{context, Template};

#[get("/<id>")]
pub fn render_post(id: String) -> Template {
    let md = read_file(format!("posts/{id}.md")).expect("");
    let content = md.content();
    let html: String = markdown::to_html(content);
    Template::render(
        "post",
        context! {
            html: html
        },
    )
}
