use crate::schema::user_sessions;

#[table_name = "user_sessions"]
#[derive(Insertable)]
pub struct SessionInfo
{
    #[column_name = "userId"]
    pub(super) user_id: u32,

    #[column_name = "sessionId"]
    pub(super) session_id: String,
}

impl Default for SessionInfo
{
    fn default() -> Self
    {
        SessionInfo
        {
            user_id: 0,
            session_id: "".to_string()
        }
    }
}

pub(super) fn generate_session_id() -> String
{
    nanoid::nanoid!(128).parse::<String>().unwrap()
}