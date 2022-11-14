use crate::schema::posts;
use diesel::{Identifiable, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use slug::slugify;

pub fn now() -> i64 {
    chrono::Utc::now().timestamp()
}

#[derive(Debug, Clone, Queryable, Identifiable, Insertable, Serialize)]
#[primary_key(slug)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub draft: bool,
    pub author: String,
    pub published: i64,
    pub tags: Vec<Option<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub tags: String,
    pub draft: bool,
}

impl Post {
    pub fn new(
        title: String,
        description: Option<String>,
        content: Option<String>,
        draft: bool,
        tags: Vec<Option<String>>,
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
            tags,
            published: now(),
        }
    }

    pub fn publish(mut self) {
        self.draft = false;
        self.published = now();
    }

    pub fn update(
        mut self,
        title: Option<String>,
        description: Option<String>,
        content: Option<String>,
    ) -> Self {
        if let Some(title) = title {
            self.title = title
        }
        self.description = description;
        self.content = content;
        self.published = now();
        self
    }
}
