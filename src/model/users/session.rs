extern crate chrono;

use crate::schema::user_sessions;
use chrono::{
    Utc,
    Duration,
    prelude,
};

#[table_name = "user_sessions"]
#[derive(Insertable)]
pub struct SessionInfo {
    #[column_name = "userId"]
    pub(super) user_id: u32,

    #[column_name = "sessionId"]
    pub(super) session_id: String,

    #[column_name = "expire"]
    pub(super) expire_date: chrono::NaiveDateTime
}

impl Default for SessionInfo {
    fn default() -> Self {
        SessionInfo {
            user_id: 0,
            session_id: "".to_string(),
            expire_date: (Utc::now() + Duration::days(1)).naive_utc(),
        }
    }
}

pub(super) fn generate_session_id() -> String {
    nanoid::nanoid!(128).parse::<String>().unwrap()
}