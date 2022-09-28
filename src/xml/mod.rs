use crate::{db::BlogDBConn, posts::database::Post, schema::posts as Posts};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::fairing::AdHoc;
use rocket_dyn_templates::{context, Template};

#[get("/sitemap.xml")]
async fn sitemap(db: BlogDBConn) -> Template {
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
        "sitemap",
        context! {
            host:"https://nullrequest.com",
            posts:posts
        },
    )
}

#[get("/index.xml")]
async fn rss(db: BlogDBConn) -> Template {
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
        "rss",
        context! {
            host:"https://nullrequest.com",
            posts:posts
        },
    )
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("xml", |rocket| async {
        rocket.mount("/", routes![sitemap, rss])
    })
}
