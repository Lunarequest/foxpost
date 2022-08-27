use super::database::User;
use rocket::{
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Error, Request,
};

#[derive(Debug, Clone, Deserialize)]
pub struct SignUp {
    pub username: String,
    pub email: String,
    pub passwd1: String,
    pub passwd2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub username: String,
    pub passwd: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Session {
    pub authkey: String,
    pub user: String,
    pub timestamp: i64,
}

/* #[async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = Error;
    async fn from_request(req: &'r Request<'_>) -> Outcome<User, Error> {
        Outcome::Success(Session)
    }
}
 */
