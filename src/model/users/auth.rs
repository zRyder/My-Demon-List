extern crate chrono;

use argonautica::Verifier;
use dotenv;
use crate::schema::user_hash;
use std::env;

///Struct representing existing user. This should be used strictly to authenticate a user.

#[table_name = "user_hash"]
#[derive(Insertable)]
pub struct AuthInfo {
    #[column_name = "userId"]
    pub(super) user_id: u32,

    #[column_name = "passwordHash"]
    pub(super) password_hash: String,
}

///Struct representing a representing user logging in. This will be used as user provided data to authenticate and preform authorized actions
#[derive(FromForm)]
pub struct LoginUser {
    ///The username of the user who is attempting to login
    pub(super) user_name: String,

    ///The non-encrypted password of the user who is trying to login in
    pub(super) password: String
}

impl Default for AuthInfo {
    fn default() -> Self {
        AuthInfo {
            user_id: 0,
            password_hash: "".to_string(),
        }
    }
}

impl LoginUser {
    pub(super) fn verify_password_hash(&self, hash: &String) -> Result<bool, argonautica::Error> {
        dotenv::dotenv().ok();

        //TO VERIFY PASSWORDS
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(hash)
            .with_password(&self.password)
            .with_secret_key(&std::env::var("SECRET_HASH").unwrap())
            .verify();

        is_valid
    }
}

pub(crate) fn is_valid_session(session_id: &str, db_conn: &crate::DbConnection) -> Option<u32> {
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
    use crate::schema::user_sessions::dsl::{
        user_sessions,
        userId,
        expire,
        sessionId,
    };

    let session_result = user_sessions.select((userId, expire)).filter(sessionId.eq(session_id)).load::<(u32, chrono::NaiveDateTime)>(&db_conn.0);

    match session_result {
        Ok(maybe_session) => {
            match maybe_session.first() {
                Some(user_session_info) => {
                    if chrono::Local::now().naive_utc() <= user_session_info.1{
                        print!("Working");
                        Some(user_session_info.0)
                    }
                    else {
                        print!("Session expired");
                        None
                    }
                }
                None => {
                    None
                }
            }
        }
        Err(_e) => {
            None
        }
    }
}
