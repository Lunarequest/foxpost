use reqwest::Client;
use rocket::{
	form::FromForm,
	http::Status,
	request::{FromRequest, Outcome},
	serde::{Deserialize, Serialize},
	Request,
};
const VERIFY_URL: &str = "https://hcaptcha.com/siteverify";

pub fn now() -> i64 {
	chrono::Utc::now().timestamp()
}

/* #[derive(Debug, Clone, Deserialize, Hcaptcha)]
pub struct SignUp {
	pub username: String,
	pub email: String,
	pub passwd1: String,
	pub passwd2: String,
	#[captcha]
	#[serde(rename(deserialize = "h-captcha-response"))]
	pub h_captcha_response: String,
} */

#[derive(Debug, Clone, FromForm)]
pub struct Login {
	pub username: String,
	pub passwd: String,
	#[field(name = "h-captcha-response")]
	pub h_captcha_response: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HcaptchaResponse {
	pub success: bool, // is the passcode valid, and does it meet security criteria you specified, e.g. sitekey?
	pub error_codes: Option<Vec<String>>, // optional: any error codes
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

impl Login {
	pub async fn valid_response(self, secret_key: &String) -> Result<(), String> {
		let params = [
			("response", &self.h_captcha_response),
			("secret", secret_key),
		];
		let client = Client::new();
		let response = match client.post(VERIFY_URL).form(&params).send().await {
			Ok(resp) => resp,
			Err(e) => {
				eprintln!("{e}");
				return Err("Unkown error making request to hcaptcha".to_string());
			}
		};
		let json = match response.json::<HcaptchaResponse>().await {
			Ok(json) => json,
			Err(e) => {
				eprintln!("{e}");
				return Err("Unkown error parsing response form hcaptcha".to_string());
			}
		};
		if json.success == true {
			Ok(())
		} else {
			match json.error_codes {
				Some(error_codes) => Err(error_codes.join(" ")),
				None => Err("Unkown error returned of error".to_string()),
			}
		}
	}
}
