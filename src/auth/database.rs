use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub passwd: String,
    pub salt: String,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub passwd: String,
    pub salt: String,
}

impl NewUser {
    async fn new(
        &mut self,
        username: String,
        email: String,
        passwd1: String,
        passwd2: String,
        salt: &[u8],
    ) -> Result<NewUser, &'static str> {
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
                return Err("unable to hash password");
            }
            Ok(passwd) => {
                self.passwd = passwd.to_string();
            }
        }
        self.email = email;
        self.username = username;
        self.salt = salt.to_string();
        return Ok(self.to_owned());
    }
}

impl User {
    pub fn verify_password(&self, password: String) -> bool {
        let argon2 = Argon2::default();
        match PasswordHash::new(self.passwd.as_str()) {
            Ok(hash) => match argon2.verify_password(password.as_bytes(), &hash) {
                Ok(_) => true,
                Err(_) => false,
            },
            Err(e) => {
                eprintln!("{e}");
                false
            }
        }
    }
}
