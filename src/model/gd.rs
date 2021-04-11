extern crate dash_rs;
extern crate reqwest;
extern crate serde_json;
extern crate chrono;

use dash_rs::
{
    model::
    {
        level::
        {
            ListedLevel,
        },
        creator::Creator,
    },

    request::
    {
        level::
        {
            LevelsRequest,
        }
    },

    response::
    {
        ResponseError,
    },

    response,
    Thunk, ThunkContent, Base64Decoded,
};

use reqwest::
{
    blocking::Client,
    Error,
};

use rocket::
{
    http::
    {
        ContentType, Status,
    },
    logger::error,
    Request, Response,
};

use serde_json::
{
    Map, Value,
};

use chrono::prelude::*;

/**
  Models a Geometry Dash in game search by making a search for a given &str 'search_string'. The request is posted to http://boomlings.com/databases/getGJLevels21.php endpoint. A Result is returned that wraps either the sucessful result of the raw response data as string or returns the reqwest error that resulted in the failure of the POST request
**/

pub fn prepare_search_request(search_string: &str, page: u32) -> Result<String, Error>
{
    let request = LevelsRequest::default(); //Request object for getGJLevels21
    let get_gj_levels21_endpoint = LevelsRequest::to_url(&request); //URL to the getGJLevels21 endpoint from dash-rs

    let request_data = request // gets parameters for the post request
        .search(search_string)
        .page(page-1)
        .to_string();

    let client = Client::new();
    let response = client
        .post(&get_gj_levels21_endpoint )
        .body(request_data)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send();

    match response
    {
        Ok(response) =>
            {
                response.text()
            },
        Err(err) => Err(err)
    }
}

/**
  Helper function that calls the parse_get_gj_levels_response function found in the dash-rs library. Function returns a result that contains either the successful list of levels Vec<ListedLevel> or the custom dash-rs response error, this is unlikely to happen but can happen due to a malformed unprocessed response stream.
**/

pub fn process_levels21_response(stream: &str) -> Result<Vec<ListedLevel>, ResponseError>
{
    response::parse_get_gj_levels_response(stream)
}

/**
  returns the description of any given level, it takes in a optional thunk that would ideally contain the encrypted level description, if the description can be decoded then it is wrapped in an Option<String> and returned, else None.
**/

pub fn get_level_description<'a>(thunk: &Option<Thunk<'a, Base64Decoded<'a>>>) -> Option<String>
{
    match thunk
    {
        Some(thunk) =>
            {
                Some(match thunk
                {
                    Thunk::Processed(processed) => //Already decoded description, route may not be possible
                        {
                            processed.0.to_string()
                        },
                    Thunk::Unprocessed(unprocessed) => //case to decode description
                        {
                            Base64Decoded::from_unprocessed(unprocessed).unwrap().0.to_string()
                        },
                })
            },
        None => None, //Occurs if the level has no description
    }
}

pub fn get_creator_name<'a>(creator: &Creator) -> String
{
    creator.name.to_string()
}

// impl rocket::response::Responder<'_> for Vec<ListedLevel<'_>>
// {
//     fn respond_to(self, request: &Request<'r>) -> rocket::response::Result<'r>
//     {
//
//         let json = serde_json::to_string(&self);
//
//         match json
//         {
//             Ok(stream) =>
//                 {
//                     Response::build()
//                         .raw_header("date", format!("{}", Local::now()))
//                         .header(ContentType::new("application", "json"))
//                         .sized_body(stream)
//                         .status(Status::Ok)
//                         .ok()
//                 },
//             Err(err) =>
//                 {
//                     Response::build()
//                         .raw_header("date", format!("{}", Local::now()))
//                         .header(ContentType::new("application", "text"))
//                         .sized_body(err)
//                         .status(Status::NotFound)
//                         .ok()
//                 }
//         }
//     }
// }
