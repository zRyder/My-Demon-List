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
    3) Check that Password contains number and symbol
    4) Hash password
    5) ensure that the email address is valid
    6) verify that the GD username and password supplied belong to a valid GD account
    7) Make sure the GDUserID isn't already in use
    8) Generate UserID
    8) Send verification email maybe?
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

    if create_info.password_has_number() && create_info.password_has_symbol()
    {

    }

    let user = user::DBUser{
        user_id: 1,
        user_name: create_info.user_name.clone(),
        password_hash: create_info.password.clone(),
        email: create_info.email.clone(),
        gd_user_id: "16".to_string()
    };

    diesel::insert_into(schema::users::table).values(user).execute(&db_conn.0);

    ApiResponse
    {
        json: Json("Success".to_string()),
        status: Status::Ok,
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