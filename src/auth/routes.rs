use super::{
    database::{NewUser, User},
    forms::{Login, SignUp},
};
use crate::{
    db,
    schema::{users, users::dsl::*},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::{json, Json, Value};

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
pub async fn login(db: db::BlogDBConn, login: Json<Login>) -> Result<Json<User>, &'static str> {
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
            true => Ok(Json(user)),
        },
    }
}
