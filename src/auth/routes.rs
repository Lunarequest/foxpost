use super::{
    database::{NewUser, User},
    forms::{now, Login, Session, SignUp},
};
use crate::{
    db,
    schema::{users, users::dsl::*},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    http::{Cookie, CookieJar, SameSite},
    serde::json::{json, Json, Value},
};

#[post("/signup", data = "<signup>")]
pub async fn signup(db: db::BlogDBConn, signup: Json<SignUp>) -> Result<Value, &'static str> {
    let new_user = NewUser::new(
        signup.username.clone(),
        signup.email.clone(),
        signup.passwd1.clone(),
        signup.passwd2.clone(),
    );
    match new_user {
        Err(e) => Err(e),
        Ok(user) => {
            match db
                .run(move |conn| diesel::insert_into(users).values(&user).execute(conn))
                .await
            {
                Err(_) => Err("an error occured while trying to insert into database"),
                Ok(_) => Ok(json!({"status":"Success"})),
            }
        }
    }
}

#[post("/login", data = "<login>")]
pub async fn login(
    jar: &CookieJar<'_>,
    db: db::BlogDBConn,
    login: Json<Login>,
) -> Result<Value, &'static str> {
    let login_value = login.clone();
    match db
        .run(move |conn| {
            users::table
                .filter(users::username.eq(login_value.username.clone()))
                .first::<User>(conn)
        })
        .await
    {
        Err(_) => Err("Incorrect Username or password"),
        Ok(user) => match user.verify_password(login.passwd.clone()) {
            false => Err("Incorrect Username or password"),
            true => {
                let session = Session {
                    authkey: user.id.to_string(),
                    user: user.username,
                    timestamp: now(),
                };
                let to_str = format!("{}", json!(session));
                let cookie = Cookie::build("user", to_str)
                    .path("/")
                    .same_site(SameSite::Strict)
                    .finish();

                jar.add_private(cookie);
                Ok(json!(session))
            }
        },
    }
}

#[post("/logout")]
pub async fn logout(jar: &CookieJar<'_>, _sess: Session) {
    jar.remove_private(Cookie::named("user"))
}
