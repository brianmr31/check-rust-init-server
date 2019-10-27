use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::json::{JsonValue};

#[derive(Debug)]
pub struct ApiResponse {
    pub json: JsonValue,
    pub status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        // println!("{:?}",self.status);
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[catch(500)]
pub fn internal_error() -> ApiResponse {
    ApiResponse {
        json: json!({"msg": "Ups! Something went wrong :(", "code": "ERR_500"}),
        status: Status::InternalServerError,
    }
}

#[catch(404)]
pub fn not_found(req: &Request) -> ApiResponse {
    ApiResponse {
        json: json!({"msg": format!("I couldn't find {:?}. Try something else?",req.uri() ), "code": "ERR_404"}),
        status: Status::NotFound,
    }
}
