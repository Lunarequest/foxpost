use hcaptcha::Hcaptcha;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Request,
};

pub fn now() -> i64 {
    chrono::Utc::now().timestamp()
}

#[derive(Debug, Clone, Deserialize, Hcaptcha)]
pub struct SignUp {
    pub username: String,
    pub email: String,
    pub passwd1: String,
    pub passwd2: String,
    #[captcha]
    #[serde(rename(deserialize = "h-captcha-response"))]
    pub h_captcha_response: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub username: String,
    pub passwd: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub authkey: String,
    pub user: String,
    pub isadmin: bool,
    pub timestamp: i64,
}

#[async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = &'static str;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let session_str = match cookies.get_private("user") {
            Some(cookie) => cookie.value().to_owned(),
            None => {
                return Outcome::Failure((
                    Status::Forbidden,
                    "You must be logged in to see this page",
                ))
            }
        };

        let session = match serde_json::from_str::<Session>(session_str.as_str()) {
            Ok(sess) => sess,
            Err(_) => return Outcome::Forward(()),
        };

        if now() - session.timestamp > 43200 {
            return Outcome::Failure((Status::Forbidden, "Session has timed out"));
        }
        rocket::outcome::Outcome::Success(session)
    }
}
