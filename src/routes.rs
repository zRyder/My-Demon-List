extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;
extern crate serde_json;

use super::schema;

use dash_rs::model::level::ListedLevel;
use rocket_contrib::json::{Json, JsonValue};


use rocket::{http::
             {
                 ContentType, Status,
             }, logger::error, Request, Response, response};

use crate::model::
{
    gd,
    user,
};
use rocket::response::Responder;
use chrono::prelude::*;
use rocket::request::Form;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, prelude::*};

#[get("/search/<search>?<page>")]
pub fn search<'a>(search: String, page: Option<u32>) -> ApiResponse
{

    let mut raw_data;
    match page
    {
        Some(page_number) =>
            {
                raw_data = gd::prepare_search_request(search.as_str(), page_number);
            },
        None =>
            {
                raw_data = gd::prepare_search_request(search.as_str(), 1);
            },
    };

    match raw_data
    {
        Ok(raw_data_stream) =>
            {
                let response = gd::process_levels21_response(&raw_data_stream);

                match response
                {
                    Ok(level_list) =>
                        {
                            let json = serde_json::to_string(&level_list).unwrap();
                            ApiResponse
                            {
                                json: Json(json),
                                status: Status::Ok
                            }
                        },
                    Err(err) =>
                        {
                            let json = serde_json::to_string(&err.to_string()).unwrap();
                            ApiResponse
                            {
                                json: Json(json),
                                status: Status::NotFound
                            }
                        }
                }

            },
        Err(err) =>
            {
                let json = serde_json::to_string(&err.to_string()).unwrap();
                ApiResponse
                {
                    json: Json(json),
                    status: Status::NotFound
                }
            }
    }
}

#[post("/users/create", format="form", data="<create_info>")]
pub fn create_user(db_conn: crate::DbConnection, create_info: Form<user::CreateUser>) -> ApiResponse
{
    use crate::schema::users::dsl::*;
    /*
    1) Check is Username is valid DONE
    2) Check is Username is available within the database DONE
    3) Check that Password contains number, Symbol, and 8 characters DONE
    4) Hash password DONE
    5) ensure that the email address is valid DONE
    8) Generate UserID DONE
    8) Send verification email maybe? LATER???
    */

    let mut db_user_entry = user::DBUser::new();

    if create_info.is_valid_username() && (users.select(userName).filter(userName.eq(create_info.user_name.clone())).load::<String>(&*db_conn).unwrap().len() ==0)
    {
        db_user_entry.user_name = create_info.user_name.clone();
    }
    else
    {
        return ApiResponse
        {
            json: Json("{error: invalid username}".to_string()),
            status: Status::BadRequest,
        }
    }

    if create_info.is_valid_password()
    {
        db_user_entry.password_hash = create_info.hash_password();;
    }
    else
    {
        return ApiResponse
        {
            json: Json("{error: invalid password}".to_string()),
            status: Status::BadRequest,
        }
    }

    if create_info.is_valid_email()
    {
        db_user_entry.email = create_info.email.clone();
    }
    else
    {
        return ApiResponse
        {
            json: Json("{error: invalid email address}".to_string()),
            status: Status::BadRequest,
        }
    }

    loop
    {
        match user::generate_user_id()
        {
            Ok(user_id) =>
                {
                    let user_id_check = users.select(userId).filter(userId.eq(&user_id)).load::<u32>(&*db_conn);

                    match user_id_check
                    {
                        Ok(user_id_result) =>
                            {
                                if user_id_result.len() == 0
                                {
                                    db_user_entry.user_id = user_id;
                                    break;
                                }
                            }
                        Err(e) =>
                            {
                                return ApiResponse
                                {
                                    json: Json(e.to_string()),
                                    status: Status::InternalServerError,
                                }
                            }
                    }

                }
            Err(e) =>
                {
                    return ApiResponse
                    {
                        json: Json(e.to_string()),
                        status: Status::InternalServerError,
                    }
                }
        }
    }

    return match diesel::insert_into(schema::users::table).values(db_user_entry).execute(&db_conn.0)
    {
        Ok(insert_check) =>
            {
                ApiResponse
                {
                    json: Json("message: user added successfully".to_string()),
                    status: Status::Ok,
                }
            }
        Err(e) =>
            {
                ApiResponse
                {
                    json: Json(e.to_string()),
                    status: Status::InternalServerError,
                }
            }
    }
}

pub struct ApiResponse
{
    //private Json response_json
    json: Json<String>,
    //private int status
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse
{
    fn respond_to(self, req: &Request) -> response::Result<'r>
    {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .raw_header("date", format!("{}", Local::now()))
            .header(ContentType::JSON)
            .ok()
    }
}