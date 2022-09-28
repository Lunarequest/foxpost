use super::{
    database::User,
    forms::{now, Login, Session},
};
use crate::{db, schema::users};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use hcaptcha::Hcaptcha;
use rocket::{
    form::Form,
    http::{Cookie, CookieJar, SameSite},
    request::FlashMessage,
    response::{Flash, Redirect},
    serde::json::json,
};
use rocket_dyn_templates::{context, Template};

#[cfg(debug_assertions)]
const SITE_KEY: &str = "20000000-ffff-ffff-ffff-000000000002";
#[cfg(debug_assertions)]
const SECRET_KEY: &str = "0x0000000000000000000000000000000000000000";
#[cfg(not(debug_assertions))]
const SITE_KEY: &str = std::env!("SITE_KEY");
#[cfg(not(debug_assertions))]
const SECRET_KEY: &str = std::env!("SECRET_KEY");

#[post("/login", data = "<login>")]
pub async fn login(
    jar: &CookieJar<'_>,
    db: db::BlogDBConn,
    login: Form<Login>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let login_value = login.clone();
    match login.valid_response(SECRET_KEY, None).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            return Err(Flash::error(
                Redirect::to("/users/login"),
                "Invalid captcha response",
            ));
        }
    }
    match db
        .run(move |conn| {
            users::table
                .filter(users::username.eq(login_value.username))
                .first::<User>(conn)
        })
        .await
    {
        Err(_) => Err(Flash::error(
            Redirect::to("/users/login"),
            "Incorrect Username or password",
        )),
        Ok(user) => match user.verify_password(login.passwd.clone()) {
            false => Err(Flash::error(
                Redirect::to("/users/login"),
                "Incorrect Username or password",
            )),
            true => {
                if user.confirmed {
                    let session = Session {
                        authkey: user.id.to_string(),
                        user: user.username,
                        isadmin: user.isadmin,
                        timestamp: now(),
                    };
                    let to_str = format!("{}", json!(session));
                    let cookie = Cookie::build("user", to_str)
                        .path("/")
                        .same_site(SameSite::Strict)
                        .finish();

                    jar.add_private(cookie);
                    Ok(Flash::success(Redirect::to("/"), "you are now logged in"))
                } else {
                    Err(Flash::error(
                        Redirect::to("/users/login"),
                        "account not active",
                    ))
                }
            }
        },
    }
}

#[get("/login")]
pub async fn login_get(
    sess: Option<Session>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Flash<Redirect>> {
    match sess {
        Some(_) => Err(Flash::error(
            Redirect::to(uri!(crate::index)),
            "You are already logged in",
        )),
        None => Ok(Template::render(
            "login",
            context! {title:"login",site_key:SITE_KEY, flash:flash},
        )),
    }
}

#[get("/logout")]
pub async fn logout(jar: &CookieJar<'_>, _sess: Session) -> Flash<Redirect> {
    jar.remove_private(Cookie::named("user"));
    Flash::success(Redirect::to("/"), "you are now logged out")
}
