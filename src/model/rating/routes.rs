use diesel::{ExpressionMethods, prelude::*, QueryDsl, RunQueryDsl};
use rocket::http::{Cookie, Status};
use rocket::http::Cookies;
use rocket::request::Form;
use rocket_contrib::json::Json;

use crate::model::{
    api_response,
    api_response::ApiResponse,
    rating::rate,
    users::auth,
    geometry_dash::gd,
};
use crate::schema::rating::dsl::levelId;

#[post("/rate", format="form", data="<rate_form>")]
pub fn rate_level(db_conn: crate::DbConnection, rate_form: Form<rate::RatingForm>, mut cookies: Cookies) -> ApiResponse {
    use crate::schema;
    use crate::schema::{
        rating::dsl::{
            rating,
            ratingId,
            levelId,
            userId as rating_userId
        }
    };

    if rate_form.rating <= 0 || rate_form.rating >= 11 {
        return ApiResponse {
            json: Json("Rating not within range".to_string()),
            status: Status::BadRequest
        }
    }
    if !gd::level_exists(&rate_form.level_id) {
        return ApiResponse {
            json: Json("level does not exist".to_string()),
            status: Status::NotFound
        }
    }

    match cookies.get_private("session") {
        Some(session_id_cookie) => {
            match auth::is_valid_session(session_id_cookie.value(), &db_conn) {
                Some(user_id) => {

                    match rating.select(ratingId)
                        .filter(rating_userId.eq(&user_id))
                        .filter(levelId.eq(rate_form.level_id))
                        .load::<u32>(&db_conn.0) {
                        Ok(level_rates_stream) => {
                            if level_rates_stream.len() != 0 {
                                ApiResponse {
                                    json: Json("User has already rated this level".to_string()),
                                    status: Status::Conflict
                                }
                            }
                            else {
                                let new_rating = rate::RateInfo {
                                    user_id,
                                    rating_id: rate::generate_rating_id(),
                                    level_id: rate_form.level_id.clone(),
                                    rate: rate_form.rating.clone()
                                };

                                match diesel::insert_into(schema::rating::table)
                                    .values(new_rating).execute(&db_conn.0) {
                                    Ok(_insert_rating_check) => {
                                        ApiResponse {
                                            json: Json("Level has been rated".to_string()),
                                            status: Status::Ok
                                        }
                                    }
                                    Err(e) => {
                                        api_response::database_error(e)
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            api_response::database_error(e)
                        }
                    }
                }
                None => {
                    ApiResponse {
                        json: Json("User is not logged in, session expired or does not exist".to_string()),
                        status: Status::Unauthorized
                    }
                }
            }
        }
        None => {
            ApiResponse {
                json: Json("User is not logged in".to_string()),
                status: Status::Unauthorized
            }
        }
    }
}

#[get("/rate?<level_id>")]
pub fn get_level_rating(level_id: u64, db_conn: crate::DbConnection) -> ApiResponse
{
    use crate::schema;
    use crate::schema::{
        rating::dsl::{
            rating,
            rate,
            levelId,
        },
    };

    if !gd::level_exists(&level_id)
    {
        return ApiResponse {
            json: Json("Level does not exist".to_string()),
            status: Status::NotFound
        }
    }

    match rating.select(rate).filter(levelId.eq(level_id)).load::<u8>(&*db_conn.0)
    {
        Ok(rating_stream) => {
            let mut total_rating = 0.0;
            for user_rating in &rating_stream {
                total_rating += *user_rating as f64
            }

            if total_rating == 0.0 {
                ApiResponse {
                    json: Json("0".to_string()),
                    status: Status::Ok
                }
            }
            else {
                total_rating /= rating_stream.len() as f64;
                ApiResponse {
                    json: Json((f64::trunc(total_rating * 100.0) / 100.0).to_string()),
                    status: Status::Ok
                }
            }
        }
        Err(e) => {
            api_response::database_error(e)
        }
    }
}