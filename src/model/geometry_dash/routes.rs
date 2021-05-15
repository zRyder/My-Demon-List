use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::model::api_response::ApiResponse;
use crate::model::geometry_dash;

#[get("/search/<search>?<page>")]
pub fn search<'a>(search: String, page: Option<u32>) -> ApiResponse
{

    let mut raw_data;
    match page
    {
        Some(page_number) =>
            {
                raw_data = geometry_dash::gd::prepare_search_request(search.as_str(), page_number);
            },
        None =>
            {
                raw_data = geometry_dash::gd::prepare_search_request(search.as_str(), 1);
            },
    };

    match raw_data
    {
        Ok(raw_data_stream) =>
            {
                let response = geometry_dash::gd::process_levels21_response(&raw_data_stream);

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
