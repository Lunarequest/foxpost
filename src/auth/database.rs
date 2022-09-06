use crate::schema::users;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::{Insertable, Queryable};
use rocket::serde::Serialize;

#[derive(Queryable, Clone, Serialize, Debug)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub passwd: String,
    pub isadmin: bool,
    #[serde(skip_deserializing)]
    pub salt: String,
    #[serde(skip_deserializing)]
    pub confirmed: bool,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub passwd: String,
    pub isadmin: bool,
    pub salt: String,
    pub confirmed: bool,
}

impl NewUser {
    pub fn new(
        username: String,
        email: String,
        passwd1: String,
        passwd2: String,
    ) -> Result<Self, &'static str> {
        if passwd1 != passwd2 {
            return Err("passwords do not match");
        }
        let password = passwd1.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password, &salt);
        match password_hash {
            Err(e) => {
                eprintln!("{e}");
                Err("unable to hash password")
            }
            Ok(passwd) => {
                let user = NewUser {
                    username,
                    email,
                    isadmin: false,
                    passwd: passwd.to_string(),
                    salt: salt.to_string(),
                    confirmed: false,
                };
                Ok(user)
            }
        }
    }
}

impl User {
    pub fn verify_password(&self, password: String) -> bool {
        let argon2 = Argon2::default();
        match PasswordHash::new(self.passwd.as_str()) {
            Ok(hash) => argon2.verify_password(password.as_bytes(), &hash).is_ok(),
            Err(e) => {
                eprintln!("{e}");
                false
            }
        }
    }
}
