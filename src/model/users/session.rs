extern crate chrono;

use chrono::{
    Duration,
    prelude,
    Utc,
};

use crate::schema::user_sessions;

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