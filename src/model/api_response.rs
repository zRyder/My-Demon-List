use chrono::Local;
use diesel::result::Error;
use rocket::{Request, response, Response};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket_contrib::json::Json;

pub fn database_error(e: Error) -> ApiResponse
{
    error!("Database error with message {:?}", &e);
    let mut error_code = Status::InternalServerError;

    if e.to_string().contains("Duplicate entry") {
        error_code = Status::Conflict
    }

    ApiResponse
    {
        json: Json(e.to_string()),
        status: error_code,
    }
}


pub struct ApiResponse
{
    //private Json response_json
    pub(crate) json: Json<String>,
    //private int status
    pub(crate) status: Status,
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
