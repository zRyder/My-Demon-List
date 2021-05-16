use diesel::{ExpressionMethods, prelude::*, QueryDsl, RunQueryDsl};
use rocket::http::{Cookie, Status};
use rocket::http::Cookies;
use rocket::request::Form;
use rocket_contrib::json::Json;

use crate::model::
{
    api_response,
    api_response::ApiResponse,
    rating::rate,

    users::auth
};

#[post("/rate", format="form", data="<rate_form>")]
pub fn rate_level(db_conn: crate::DbConnection, rate_form: Form<rate::RatingForm>, mut cookies: Cookies) -> ApiResponse
{
    if !rate_form.level_exists()
    {
        return ApiResponse
        {
            json: Json("level does not exist".to_string()),
            status: Status::BadRequest
        }
    }

    let session_cookie = cookies.get_private("session");
    match session_cookie
    {
        Some(session_id_cookie) => 
        {
            if auth::is_valid_session(session_id_cookie.value(), db_conn)
            {
                //DATABASE INSERTIONS HERE
                ApiResponse
                {
                    json: Json("Working".to_string()),
                    status: Status::Ok
                }
            }
            else
            {
                ApiResponse
                {
                    json: Json("Not Working".to_string()),
                    status: Status::Ok
                }
            }
        }
        None =>
        {
            ApiResponse
            {
                json: Json("User is not logged in".to_string()),
                status: Status::Unauthorized
            }
        }
    }
}