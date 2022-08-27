use crate::schema::posts;
use diesel::{Insertable, Queryable};
use rocket::serde::Deserialize;
use slug::slugify;

#[derive(Debug, Clone, Queryable, Insertable)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub draft: bool,
    pub author: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub draft: bool,
}

impl Post {
    pub fn new(
        title: String,
        description: Option<String>,
        content: Option<String>,
        draft: bool,
        author: String,
    ) -> Self {
        let slug = slugify(title.clone());
        Post {
            slug,
            title,
            description,
            content,
            draft,
            author,
        }
    }

    pub fn publish(mut self) {
        self.draft = false;
    }

    pub fn update(
        mut self,
        title: Option<String>,
        description: Option<String>,
        content: Option<String>,
    ) {
        match title {
            Some(title) => self.title = title,
            None => {}
        };
        self.description = description;
        self.content = content;
    }
}
