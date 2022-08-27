use diesel::{Insertable,Queryable};
use crate::schema::posts;

#[derive(Debug,Queryable,Insertable)]
pub struct Post {
    slug: String,
    title:String,
    description: Option<String>,
    content: Option<String>,
    draft: bool,
    author: String,
}