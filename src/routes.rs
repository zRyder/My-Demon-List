extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;

use dash_rs::model::level::ListedLevel;
use rocket_contrib::json::{Json, JsonValue};

use rocket::{http::
             {
                 ContentType, Status,
             }, logger::error, Request, Response, response};

use crate::gd;
use rocket::response::Responder;
use chrono::prelude::*;

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

pub struct ApiResponse {
    json: Json<String>,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .raw_header("date", format!("{}", Local::now()))
            .header(ContentType::JSON)
            .ok()
    }
}